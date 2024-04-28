const CHARS: &[char] = &[
    '(', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x',
    'y', 'z'];

#[derive(Debug)]
pub struct Sensors {
    pub angle: f64, // +/- pi (rad)
    pub cur_lap_time: f64, // seconds
    pub damage: usize, // points
    pub dist_from_start: f64,
    pub dist_raced: f64,
    pub fuel: f64, // litres
    pub gear: i8,
    pub last_lap: f64,
    pub opponents: [f64; 36],
    pub race_pos: u8,
    pub rpm: f64,
    pub speed_x: f64,
    pub speed_y: f64,
    pub speed_z: f64,
    pub track: [f64; 19],
    pub track_pos: f64,
    pub wheel_spin_vel: [f64; 4],
    pub z: f64,
    pub focus: [f64; 5] // 5 vectors
}

impl Sensors {
    pub fn new() -> Self {
        Sensors {
            angle: 0.0,
            cur_lap_time: 0.0,
            damage: 0,
            dist_from_start: 0.0,
            dist_raced: 0.0,
            focus: [0.0; 5],
            fuel: 0.0,
            gear: 0,
            last_lap: 0.0,
            opponents: [0.0; 36],
            race_pos: 0,
            rpm: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            speed_z: 0.0,
            track: [0.0; 19],
            track_pos: 0.0,
            wheel_spin_vel: [0.0; 4],
            z: 0.0,
        }
    }

    pub fn update(&mut self, raw_str: String) {
        let mut raw_sensor: Vec<&str> = raw_str.split(")").collect();
        let _ = raw_sensor.pop();
        
        //println!("raw_sensor.len() = {}", raw_sensor.len());
        debug_assert!(raw_sensor.len() == 19);

        let sensors: Vec<String> = raw_sensor.iter().map(|sensor| {
            sensor.to_lowercase().trim_matches(CHARS).trim().to_owned()
        }).collect();

        //for i in 0..sensors.len() {
        //    if i == 19 {
        //        println!("{:#?}", sensors[i].as_bytes())
        //    } else {
        //        println!("i = {}, str ={}", i, sensors[i]);
        //    }
        //}

        // Angle
        //println!("bytes = {:#?}", sensors[0].as_bytes());
        self.angle = sensors[0].parse::<f64>().unwrap();

        // Current lap time
        self.cur_lap_time = sensors[1].parse::<f64>().unwrap();

        // Damage
        self.damage = sensors[2].parse::<usize>().unwrap();

        // Distance from start/finish line
        self.dist_from_start = sensors[3].parse::<f64>().unwrap();

        // Total distance raced
        self.dist_raced = sensors[4].parse::<f64>().unwrap();

        // Fuel
        self.fuel = sensors[5].parse::<f64>().unwrap();

        // Gear
        self.gear = sensors[6].parse::<i8>().unwrap();

        // Last lap time
        self.last_lap = sensors[7].parse::<f64>().unwrap();

        // Opponents
        let opponents: Vec<f64> = sensors[8].split_whitespace().map(|opponent| {
            opponent.parse::<f64>().unwrap()
        }).collect();

        debug_assert!(opponents.len() == 36);

        for i in 0..36 {
            self.opponents[i] = opponents[i];
        }

        // Race position
        self.race_pos = sensors[9].parse::<u8>().unwrap();

        // RPM (rotations per minute)
        self.rpm = sensors[10].parse::<f64>().unwrap();

        // Speed along x, y and z axis
        self.speed_x = sensors[11].parse::<f64>().unwrap();
        self.speed_y = sensors[12].parse::<f64>().unwrap();
        self.speed_z = sensors[13].parse::<f64>().unwrap();

        // Track sensors
        let track: Vec<f64> = sensors[14].split_whitespace().map(|track_sensor| {
            track_sensor.parse::<f64>().unwrap()
        }).collect();

        debug_assert!(track.len() == 19);

        for i in 0..19 {
            self.track[i] = track[i];
        }

        // Track position        
        self.track_pos = sensors[15].parse::<f64>().unwrap();

        // Wheel spin velocity
        let wheel: Vec<f64> = sensors[16].split_whitespace().map(|wheel| {
            wheel.parse::<f64>().unwrap()
        }).collect();

        debug_assert!(wheel.len() == 4);

        for i in 0..4 {
            self.wheel_spin_vel[i] = wheel[i];
        }

        // Z (?)
        self.z = sensors[17].parse::<f64>().unwrap();

        // Focus sensors
        let focus: Vec<f64> = sensors[18].split_whitespace().map(|focus| {
            focus.parse::<f64>().unwrap()
        }).collect();
        debug_assert!(focus.len() == 5);

        for i in 0..5 {
            self.focus[i] = focus[i];
        }
    }
}