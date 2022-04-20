use crate::cli::{ClientConfig, ClientSide, ServerConfig};
use core_foundation::{
    base::{kCFAllocatorDefault, ToVoid},
    boolean::kCFBooleanTrue,
    runloop::{kCFRunLoopCommonModes, CFRunLoop},
    string::{kCFStringEncodingUTF8, CFStringCreateWithCString, CFStringRef},
};
use core_graphics::{
    display::{CGDisplay, CGSSetConnectionProperty, _CGSDefaultConnection},
    event::{
        CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventTapProxy,
        CGEventType,
    },
    geometry::CGPoint,
};
use local_ip_address::local_ip;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct Server {
    config: ServerConfig,
    addr: IpAddr,
    // listener: TcpListener,
}

impl Server {
    pub fn from_config(config: ServerConfig) -> Self {
        let addr = local_ip().unwrap();
        Self {
            config,
            addr,
            // listener: TcpListener::bind(format!("{}:4242", addr)).unwrap(),
        }
    }

    pub fn run(&self) {
        println!("Runnig hijack server on: {:?}", self.addr);
        // improve this log
        println!("Server configuration: {:?}", self.config);

        let (mouse_tx, mouse_rx) = mpsc::channel();

        // mouse monitor thread
        thread::spawn(move || {
            dbg!("mouse thread spawned");
            let cur_run_loop = CFRunLoop::get_current();
            match CGEventTap::new(
                CGEventTapLocation::HID,
                CGEventTapPlacement::HeadInsertEventTap,
                CGEventTapOptions::Default,
                vec![CGEventType::MouseMoved /*CGEventType::LeftMouseDown*/],
                |_a, _b, event| {
                    mouse_tx.send(event.location()).unwrap();
                    None
                },
            ) {
                Ok(tap) => unsafe {
                    let loop_source = tap.mach_port.create_runloop_source(0).unwrap();
                    cur_run_loop.add_source(&loop_source, kCFRunLoopCommonModes);
                    tap.enable();
                    CFRunLoop::run_current();
                },
                Err(_) => panic!("event tap panicked"),
            }
        });

        // TCP thread
        let addr = self.addr;
        thread::spawn(move || {
            let listener = TcpListener::bind(format!("{}:4242", addr)).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(mut tcp_stream) => {
                        let mut data = [0 as u8; 50]; // using 50 byte buffer
                        tcp_stream.read(&mut data).expect("Failed to read tcp data");
                        Server::handle_msg(&data);
                    }
                    Err(e) => panic!("Error in tcp stream"),
                }
            }
        });

        let mut mouse_owner = "server";
        for mouse_pos in mouse_rx {
            // println!("mouse_pos: {}, {}", mouse_location.x, mouse_location.y);
            if mouse_owner == "server" && mouse_pos.x == 0.0 {
                mouse_owner = "client";
                println!("the mouse is in {}", mouse_owner);
                CGDisplay::warp_mouse_cursor_position(CGPoint::new(2558.0, mouse_pos.y)).unwrap();
            } else if mouse_owner == "client" && mouse_pos.x > 2559.0 {
                mouse_owner = "server";
                println!("the mouse is in {}", mouse_owner);
                CGDisplay::warp_mouse_cursor_position(CGPoint::new(1.0, mouse_pos.y)).unwrap();
            }
        }
    }

    fn handle_msg(msg: &[u8]) {
        let decoded_msg = String::from_utf8(msg.to_vec()).expect("Failed to decode message");
        match decoded_msg.chars().collect::<Vec<char>>()[0] {
            'c' => Server::handle_first_conn(decoded_msg),
            _ => println!("Unknow message"),
        }
    }

    fn handle_first_conn(msg: String) {
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
