use crate::sensors::Sensors;
use crate::actuators::Actuators;

pub struct Driver {
    pub sensors: Sensors,
    pub actuators: Actuators
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            sensors: Sensors::new(),
            actuators: Actuators::new()
        }
    }

    pub fn drive(&mut self) {
        if self.sensors.speed_x < 25.0 {
            self.actuators.gear = 1;
        } else {
            self.actuators.gear = 2;
        }

        if self.sensors.speed_x < 50.0 {
            self.actuators.accel = 0.5;
        } else {
            self.actuators.accel = 0.0;
        }

        if self.sensors.track_pos != 0.0 {
            self.actuators.steer = self.sensors.angle * (-1.0);
        }
    }
}