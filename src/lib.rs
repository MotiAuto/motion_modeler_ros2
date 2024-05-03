use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Wheel
{
    pub fl:f32,
    pub fr:f32,
    pub rl:f32,
    pub rr:f32,
}

impl Wheel {
    pub fn new(fl_:f32, fr_:f32, rl_:f32, rr_:f32)->Wheel
    {
        Wheel { fl: fl_, fr: fr_, rl: rl_, rr: rr_ }
    }

    pub fn serialize(self)->String
    {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OneMotor
{
    pub pow:f32,
}

impl OneMotor {
    pub fn new(pow_:f32)->OneMotor
    {
        OneMotor{pow:pow_}
    }

    pub fn serialize(self)->String
    {
        serde_json::to_string(&self).unwrap()
    }
}