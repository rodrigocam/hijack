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

use crate::macos;
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

    pub fn run(&self) {
        println!("Runnig hijack server on: {:?}", self.addr);
        // improve this log
        println!("Server configuration: {:?}", self.config);

        let (mouse_tx, mouse_rx) = mpsc::channel();
        let (comm_tx, comm_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

        spawn_register_thread(self.addr, Arc::clone(&self.client_registry));

        #[cfg(target_os = "macos")]
        macos::spawn_mouse_thread(mouse_tx);

        let client_registry = Arc::clone(&self.client_registry);

        // communication thread
        thread::spawn(move || {
            for activated_client_name in comm_rx {
                println!("moving to another screen");
                let client_name = activated_client_name.as_str();
                let r = client_registry.lock().unwrap();
                let client_addr = r.get(client_name).unwrap();
                let mut client_stream = TcpStream::connect(client_addr).unwrap();
                let wr = client_stream.write(format!("mouse entered").as_bytes());
                println!("{:?}", wr);
                // .unwrap();
            }
        });

        // let mut mouse_owner = "server";
        for mouse_pos in mouse_rx {
            // println!("mouse_pos: {}, {}", mouse_location.x, mouse_location.y);
            for client in &self.config.clients {
                if client.is_active(mouse_pos) {
                    comm_tx.send(client.name.clone()).unwrap();
                }
            }
            // if mouse_owner == "server" && mouse_pos.x == 0.0 {
            //     mouse_owner = "client";
            //     println!("the mouse is in {}", mouse_owner);
            //     CGDisplay::warp_mouse_cursor_position(CGPoint::new(2558.0, mouse_pos.y)).unwrap();
            // } else if mouse_owner == "client" && mouse_pos.x > 2559.0 {
            //     mouse_owner = "server";
            //     println!("the mouse is in {}", mouse_owner);
            //     CGDisplay::warp_mouse_cursor_position(CGPoint::new(1.0, mouse_pos.y)).unwrap();
            // }
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
