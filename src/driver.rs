use std::io::Write;

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
            actuators: Actuators::new(),
        }
    }

    pub fn drive(&mut self) {
        // Write current sensor data to file
        let file = std::fs::File::options()
            .read(true)
            .append(true)
            .open("track.csv");

        if let Ok(mut data) = file {
            let _ = data.write({
                format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
                self.sensors.dist_from_start,
                self.sensors.dist_raced,
                self.sensors.angle,
                self.sensors.track_pos,
                self.actuators.steer,
                self.sensors.track[0],
                self.sensors.track[1],
                self.sensors.track[2],
                self.sensors.track[3],
                self.sensors.track[4],
                self.sensors.track[5],
                self.sensors.track[6],
                self.sensors.track[7],
                self.sensors.track[8],
                self.sensors.track[9],
                self.sensors.track[10],
                self.sensors.track[11],
                self.sensors.track[12],
                self.sensors.track[13],
                self.sensors.track[14],
                self.sensors.track[15],
                self.sensors.track[16],
                self.sensors.track[17],
                self.sensors.track[18],
            ).as_bytes()
            });
        } else {
            panic!("Failed to open track file!");
        }

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
            let target_angle = self.sensors.angle - 
                                    self.sensors.track_pos * 0.5;
            const STEER_LOCK: f64 = 0.366519;

            //self.actuators.steer = (self.sensors.track_pos * (-1.0)) / 2.0;
            self.actuators.steer = target_angle / STEER_LOCK;
        }
    }
}