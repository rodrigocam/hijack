use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub struct ThreadMessage;

pub type ClientChannels = HashMap<String, mpsc::Sender<ThreadMessage>>;

pub struct Server {
    client_channels: Arc<Mutex<ClientChannels>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            client_channels: Arc::new(Mutex::new(ClientChannels::new())),
        }
    }

    fn spawn_client_connection_thread(&self) {
        let listener = TcpListener::bind("0.0.0.0:4242").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let client_channels = Arc::clone(&self.client_channels);
                    thread::spawn(move || {
                        // connection succeeded
                        Self::handle_new_connection(stream, client_channels);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    fn handle_new_connection(mut stream: TcpStream, client_channels: Arc<Mutex<ClientChannels>>) {
        let mut buffer = [0 as u8; 250];
        match stream.read(&mut buffer) {
            Ok(_size) => {
                let decoded_msg = std::str::from_utf8(&buffer).unwrap();
                let decoded_msg = decoded_msg.trim_matches(char::from(0));
                let client_info: Vec<&str> = decoded_msg.split(',').collect();

                // handle registering
                let (client_tx, client_rx) = mpsc::channel();
                let mut registry = client_channels.lock().unwrap();

                if client_info.len() == 2 && !registry.contains_key(client_info[0]) {
                    println!("Registering client `{}`", client_info[0]);
                    registry.insert(client_info[0].to_string(), client_tx);
                }
                for message in client_rx {
                    println!("Messag received: {:?}", message);
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
        {}
    }

    pub fn run(&self) {
        self.spawn_client_connection_thread();

        let (mouse_tx, mouse_rx) = mpsc::channel();

        #[cfg(target_os = "macos")]
        use crate::macos;
        macos::spawn_mouse_observer_thread(mouse_tx);

        #[cfg(target_os = "windows")]
        windows::spawn_mouse_observer_thread();
    }
}
