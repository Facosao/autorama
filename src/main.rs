pub mod scr_net;
pub mod driver;
pub mod sensors;
pub mod actuators;

fn main() {
    let socket = scr_net::connect();
    let id: &str = "SCR";
    let mut angles: [f64; 19] = [0.0; 19];
    for i in 0..19 {
        angles[i] = (-90 + (i as isize * 10)) as f64;
    }
    
    let mut raw_angle: String = String::new();

    for value in angles {
        raw_angle += &(" ".to_owned() + &value.to_string());
    }
    
    let pre_str = id.to_string() + "(init" + &raw_angle + ")";

    println!("{}", pre_str);

    let final_str = pre_str.as_bytes();
    let result = socket.send(final_str);

    match result {
        Ok(_) => {}

        Err(error) => {
            println!("Failed to send data! {}", error);
        }
    }

    loop {
        
    }
}
