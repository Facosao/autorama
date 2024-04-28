pub struct Actuators {
    pub accel: f64,
    pub brake: f64,
    pub gear: i8,
    pub steer: f64,
    pub clutch: f64,
    pub focus: f64,
    pub meta: u8
}

impl Actuators {
    pub fn new() -> Self {
        Actuators {
            accel: 0.0,
            brake: 0.0,
            gear: 0,
            steer: 0.0,
            clutch: 0.0,
            focus: 0.0,
            meta: 0
        }
    }

    pub fn serialize(&self) -> String {
        format!("(accel {})(brake {})(gear {})(steer {})(clutch {})(focus {})(meta {})",
            self.accel,
            self.brake,
            self.gear,
            self.steer,
            self.clutch,
            self.focus,
            self.meta
        )
    }
}

// Sending (accel 0.151806)(brake 0)(gear 3)(steer 0.0749473)(clutch 0)(focus 0)(meta 0)