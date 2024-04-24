pub struct Sensors {
    angle: f64, // +/- pi (rad)
    cur_lap_time: f64, // seconds
    damage: usize, // points
    dist_from_start: f64,
    dist_raced: f64,
    focus: [f64; 5], // 5 vectors
    fuel: f64, // litres
    gear: i8,
    last_lap: f64,
    opponents: [f64; 36],
    race_pos: u8,
    rpm: u16,
    speed_x: f64,
    speed_y: f64,
    speed_z: f64,
    track: [f64; 19],
    track_pos: f64,
    wheel_spin_vel: [f64; 4],
    z: f64
}

impl Sensors {
    fn new() -> Self {
        Sensors {
            angle: 0.0,
            cur_lap_time: 0.0,
            damage: 0,
            dist_from_start: 0.0,
            dist_raced: 0.0,
            focus: [0.0; 5],
            fuel: 0.0,
            gear: 1,
            last_lap: 0.0,
            opponents: todo!(),
            race_pos: todo!(),
            rpm: todo!(),
            speed_x: todo!(),
            speed_y: todo!(),
            speed_z: todo!(),
            track: todo!(),
            track_pos: todo!(),
            wheel_spin_vel: todo!(),
            z: todo!(),
        }
    }
}