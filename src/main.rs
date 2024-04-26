pub mod scr_net;
pub mod driver;
pub mod sensors;
pub mod actuators;

use scr_net::SCRNet;

fn main() {
    let socket = scr_net::connect();
    let id: &str = "SCR";
    let mut angles: [f64; 19] = [0.0; 19];
    for i in 0..19 {
        angles[i] = (-90 + (i as isize * 10)) as f64;
    }
    
    socket.scr_init(id.to_owned(), angles);

    loop {
        println!("{}", socket.scr_recv());
    }
}
