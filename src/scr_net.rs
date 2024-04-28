use std::net::UdpSocket;

const LOCAL_ADDR: &str = "127.0.0.1:41234";

pub trait SCRNet {
    fn scr_recv(&self) -> String;
    fn scr_init(&self, id: String, angles: [f64; 19]);
}

impl SCRNet for UdpSocket {
    fn scr_init(&self, id: String, angles: [f64; 19]) {
        let mut raw_angle: String = String::new();

        for value in angles {
            raw_angle += &(" ".to_owned() + &value.to_string());
        }
        
        let pre_str = id.to_string() + "(init" + &raw_angle + ")";

        println!("{}", pre_str);

        let final_str = pre_str.as_bytes();
        //let result = self.send(final_str);

        if let Err(e) = self.send(final_str) {
            panic!("Failed to send init data! ({})", e);
        }
        
        if self.scr_recv() == "***identified***" {
            println!("Successfully connected to server");
        } else {
            panic!("Failed to connect to server");
        }
    }

    fn scr_recv(&self) -> String {
        let mut buffer: [u8; 1024] = [0; 1024];
        let received = self.recv(&mut buffer);
        match received {
            Ok(len_received) => {
                if len_received == 0 {
                    panic!("scr_recv_len == 0");
                }
                
                let recv_str = std::str::from_utf8(&buffer).unwrap();
                return recv_str.trim_end_matches(char::from(0)).to_owned();
            }

            Err(_) => {
                panic!("Failed to receive data!");
            }
        }    
    }
}

pub fn connect() -> UdpSocket {
    if let Ok(sock) = UdpSocket::bind(LOCAL_ADDR) {
        if let Ok(_) = sock.connect("127.0.0.1:3001") {
            return sock;
        } else {
            panic!("Failed to connect socket!");
        }
    } else {
        panic!("Failed to create socket!");
    }
}

pub fn scr_init() {}

