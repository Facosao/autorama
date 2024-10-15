pub mod scr_net;
pub mod driver;
pub mod sensors;
pub mod actuators;

use scr_net::SCRNet;
use driver::Driver;

fn main() {
    let socket = scr_net::connect();
    let id: &str = "SCR";
    let mut angles: [f64; 19] = [0.0; 19];
    // angles[0] = -90.0;
    // angles[1] = 90.0;
    for i in 0..19 {
       angles[i] = (-90 + (i as isize * 10)) as f64;
    }
    
    let mut driver = Driver::new();

    socket.scr_init(id.to_owned(), angles);

    loop {
        let response = socket.scr_recv();

        match response.as_str() {
            "***shutdown***" => break,
            "***restart***" => continue,
            _ => {
                driver.sensors.update(response);
                driver.drive();
            }
        }

        if let Err(_) = socket.send(driver.actuators.serialize().as_bytes()) {
            println!("Failed to send actuators data!");
        }
    }
}
