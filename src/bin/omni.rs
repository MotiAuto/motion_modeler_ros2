use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::{common_interfaces::{geometry_msgs, std_msgs}, RosString}, pr_info
};

use motion_modeler_ros2::Wheel;

fn main()->Result<(), DynError>
{
    let ctx = Context::new()?;
    let node = ctx.create_node("omni_motion_modeler", None, Default::default())?;
    let mut selector = ctx.create_selector()?;
    let log = Logger::new(node.get_name().unwrap().as_str());

    let subscriber = node.create_subscriber::<geometry_msgs::msg::Twist>("/cmd_vel", None)?;
    let publisher = node.create_publisher::<std_msgs::msg::String>("/wheel", None)?;

    let diagonal = (2.0_f32).sqrt() / 2.0;

    selector.add_subscriber(
        subscriber, 
    Box::new(move |msg|{
        let mut wheel = Wheel::new(0.0, 0.0, 0.0, 0.0);
        let x = msg.linear.x as f32;
        let y = msg.linear.y as f32;
        let ro = msg.angular.z as f32;
        wheel.fl = x * diagonal - y * diagonal + 0.5 * ro;
        wheel.fr = -x * diagonal - y * diagonal + 0.5 * ro;
        wheel.rl = x * diagonal + y * diagonal + 0.5 * ro;
        wheel.rr = - x * diagonal + y * diagonal + 0.5 * ro;

        let mut send_msg = std_msgs::msg::String::new().unwrap();
        send_msg.data = RosString::new(wheel.serialize().as_str()).unwrap();

        let _ = publisher.send(&send_msg).unwrap();
    }));

    pr_info!(log, "Start OmniMotionModelerROS2");

    loop {
        selector.wait()?;
    }
}