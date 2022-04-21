use std::collections::HashMap;
use std::io::Read;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub type ClientRegistry = HashMap<String, TcpStream>;

pub fn run_register_thread(ip: IpAddr, registry: Arc<Mutex<ClientRegistry>>) {
    thread::spawn(move || {
        let listener = TcpListener::bind(format!("{}:4242", ip)).unwrap();
        for stream in listener.incoming() {
            let mut tcp_stream = stream.unwrap();
            let mut buffer = [0 as u8; 250];
            tcp_stream.read(&mut buffer).unwrap();
            let decoded_msg = std::str::from_utf8(&buffer).unwrap();
            let client_info: Vec<&str> = decoded_msg.split(',').collect();

            // handle registering
            let mut r = registry.lock().unwrap();

            if client_info.len() == 2 && !r.contains_key(client_info[0]) {
                println!(
                    "Registering client `{}` with addr `{}`",
                    client_info[0], client_info[1]
                );
                r.insert(
                    client_info[0].to_string(),
                    TcpStream::connect(client_info[1]).unwrap(),
                );
            }
        }
    });
}
