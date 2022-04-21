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
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::register::*;

pub struct Server {
    config: ServerConfig,
    addr: IpAddr,
    client_registry: Arc<Mutex<ClientRegistry>>,
}

impl Server {
    pub fn from_config(config: ServerConfig) -> Self {
        let addr = local_ip().unwrap();
        Self {
            config,
            addr,
            client_registry: Arc::new(Mutex::new(ClientRegistry::new())),
        }
    }

    // pub fn is_client_in_config(&self, client_id: &String) -> bool {
    //     self.config
    //         .clients
    //         .iter()
    //         .filter(|cg| cg.name == *client_id)
    //         .count()
    //         > 0
    // }

    // pub fn is_client_registered(&self, client_id: &String) -> bool {
    //     self.registered_clients.contains_key(client_id)
    // }

    // pub fn try_register_client(&mut self, client_id: &String, client_addr: &String) -> bool {
    //     dbg!("Trying to register client: `{}`", client_id);
    //     if self.is_client_in_config(client_id) && !self.is_client_registered(client_id) {
    //         self.registered_clients
    //             .insert(client_id.clone(), TcpStream::connect(client_addr).unwrap());
    //         return true;
    //     }
    //     false
    // }

    // pub fn handle_client_register(&mut self) {
    //     for client_info in &self.register_channel {}
    // }

    // /// Run the thread responsible for listening to clients registering
    // pub fn run_client_register_thread(&mut self) {
    //     let gate_addr = self.addr;
    //     // let (tx, rx) = mpsc::channel();
    //     // self.register_channel = Some(rx);
    //     let data = Arc::new(Mutex::new(&self));

    //     thread::spawn(move || {
    //         let listener = TcpListener::bind(format!("{}:4242", gate_addr)).unwrap();
    //         for stream in listener.incoming() {
    //             match stream {
    //                 Ok(mut tcp_stream) => {
    //                     let mut buffer = [0 as u8; 250];
    //                     tcp_stream.read(&mut buffer).unwrap();
    //                     let decoded_msg =
    //                         String::from_utf8(buffer.to_vec()).expect("Failed to decode message");
    //                     let client_info = decoded_msg
    //                         .split(',')
    //                         .map(|s| s.to_string())
    //                         .collect::<Vec<String>>();
    //                     data.lock();
    //                     // tx.send(client_info).unwrap();
    //                 }
    //                 Err(e) => panic!("Error in tcp stream"),
    //             }
    //         }
    //     });
    // }

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

        run_register_thread(self.addr, Arc::clone(&self.client_registry));

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

    // fn handle_msg(msg: &[u8]) {
    //     let decoded_msg = String::from_utf8(msg.to_vec()).expect("Failed to decode message");
    //     match decoded_msg.chars().collect::<Vec<char>>()[0] {
    //         'c' => Server::handle_first_conn(decoded_msg),
    //         _ => println!("Unknow message"),
    //     }
    // }
}
