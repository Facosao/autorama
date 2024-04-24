use crate::sensors::Sensors;
use crate::actuators::Actuators;

pub struct Driver {
    sensors: Sensors,
    actuators: Actuators
}

impl Driver {
    //pub fn new() -> Self {
    //    Driver {  }
    //}

    pub fn init() -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::with_capacity(19);
        for i in 0..19 {
            vec[i] = (-90 * i as isize + 10) as f32;
        }
        return vec;
    }

    pub fn drive(sensors: String) -> String {
        return String::new();
    }

    pub fn shutdown() {

    }

    pub fn restart() {

    }
}