use crate::cli::{ClientConfig, ClientSide, ServerConfig};
use local_ip_address::local_ip;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    config: ServerConfig,
    addr: IpAddr,
    listener: TcpListener,
}

impl Server {
    pub fn from_config(config: ServerConfig) -> Self {
        let addr = local_ip().unwrap();
        Self {
            config,
            addr,
            listener: TcpListener::bind(format!("{}:4242", addr)).unwrap(),
        }
    }

    pub fn run(&self) {
        println!("Runnig hijack server on: {:?}", self.addr);
        // improve this log
        println!("Server configuration: {:?}", self.config);

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut tcp_stream) => {
                    let mut data = [0 as u8; 50]; // using 50 byte buffer
                    tcp_stream.read(&mut data).expect("Failed to read tcp data");
                    self.handle_msg(&data);
                }
                Err(e) => panic!("Error in tcp stream"),
            }
        }
    }

    fn handle_msg(&self, msg: &[u8]) {
        let decoded_msg = String::from_utf8(msg.to_vec()).expect("Failed to decode message");
        match decoded_msg.chars().collect::<Vec<char>>()[0] {
            'c' => self.handle_first_conn(decoded_msg),
            _ => println!("Unknow message"),
        }
    }

    fn handle_first_conn(&self, msg: String) {
        println!(
            "{} connected to the server",
            msg.split(' ').collect::<Vec<&str>>()[1]
        );
    }
}

pub struct Client {
    name: String,
    server_addr: IpAddr,
}

impl Client {
    pub fn new(name: String, server_addr: IpAddr) -> Self {
        Self { name, server_addr }
    }

    pub fn run(self) {
        let mut stream = TcpStream::connect(format!("{}:4242", self.server_addr))
            .expect("Error while connecting to server");

        println!("Connected to: {}", self.server_addr);
        stream
            .write(format!("c: {}", self.name).as_bytes())
            .expect("Error while talking to the server");
        let mut data = [0 as u8; 50]; // using 50 byte buffer
        loop {
            match stream.read(&mut data) {
                Ok(size) => {
                    if size > 0 {
                        println!("received msg from server");
                    }
                }
                Err(e) => println!("error listening to server"),
            }
        }
    }
}
