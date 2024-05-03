use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::{common_interfaces::std_msgs, RosString}, pr_info
};

use motion_modeler_ros2::OneMotor;

fn main()->Result<(), DynError>
{
    let ctx = Context::new()?;
    let node = ctx.create_node("one_motor_motion_modeler", None, Default::default())?;
    let mut selector = ctx.create_selector()?;
    let log = Logger::new(node.get_name().unwrap().as_str());

    let subscriber = node.create_subscriber::<std_msgs::msg::Float32>("/power", None)?;
    let publisher = node.create_publisher::<std_msgs::msg::String>("/motor", None)?;

    selector.add_subscriber(
        subscriber, 
    Box::new(move |msg|{
        let mut motor = OneMotor::new(0.0);
        motor.pow = msg.data;
        let mut send_msg = std_msgs::msg::String::new().unwrap();
        send_msg.data = RosString::new(motor.serialize().as_str()).unwrap();

        let _ = publisher.send(&send_msg).unwrap();
    }));

    pr_info!(log, "Start OneMotionModelerROS2");

    loop {
        selector.wait()?;
    }
}