use crate::sensors::Sensors;
use crate::actuators::Actuators;

pub struct Driver {
    pub sensors: Sensors,
    pub actuators: Actuators
}

impl Driver {
    //pub fn new() -> Self {
    //    Driver {  }
    //}

    pub fn drive(sensors: String) -> String {
        return String::new();
    }

    pub fn shutdown() {

    }

    pub fn restart() {

    }
}