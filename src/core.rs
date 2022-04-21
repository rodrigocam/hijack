use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::thread;

pub struct Client {
    name: String,
    server_addr: IpAddr,
}

impl Client {
    pub fn new(name: String, server_addr: IpAddr) -> Self {
        Self { name, server_addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind("0.0.0.0:4243").unwrap();
        // thread::spawn(move || for stream in listener.incoming() {});
        let mut stream = TcpStream::connect(format!("{}:4242", self.server_addr))
            .expect("Error while connecting to server");

        println!("Connected to: {}", self.server_addr);
        stream
            .write(format!("{},{}", self.name, "0.0.0.0:4243").as_bytes())
            .expect("Error while talking to the server");
        loop {}
        // let mut data = [0 as u8; 50]; // using 50 byte buffer
        // loop {
        //     match stream.read(&mut data) {
        //         Ok(size) => {
        //             if size > 0 {
        //                 println!("received msg from server");
        //             }
        //         }
        //         Err(e) => println!("error listening to server"),
        //     }
        // }
    }
}
