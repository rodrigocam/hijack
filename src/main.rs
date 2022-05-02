#[macro_use]
extern crate lazy_static;
use local_ip_address::local_ip;

use clap::Parser;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;

pub mod cli;
pub mod client;
pub mod windows;
pub mod teste;
use teste::*;
use windows::{set_mouse_hook, set_raw_input};
use crate::cli::Cli;


fn main() -> std::io::Result<()> {
    // set_mouse_hook();
    _main();
    let args = Cli::parse();

    match args.server_addr {
        Some(ip) => {
            // we are running a client
            let mut stream = TcpStream::connect(format!("{}:4242", ip))?;
            let msg = b"name=test";
            stream.write(msg)?;

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
        None => {
            let local_ip = local_ip().unwrap();

            let client = args.client.unwrap();

            println!("Runnig hijack server on: {:?}", local_ip);
            println!(
                "Client: {} configured at {:?} position.",
                client.name, client.side
            );
            {
                let listener = TcpListener::bind("0.0.0.0:4242")?;

                for stream in listener.incoming() {
                    match stream {
                        Ok(mut tcp_stream) => {
                            let mut data = [0 as u8; 50]; // using 50 byte buffer
                            tcp_stream.read(&mut data)?;
                            let client_name =
                                from_utf8(&data).unwrap().split('=').collect::<Vec<&str>>()[1];
                            println!(
                                "Client `{}` connected with addr `{}`",
                                client_name,
                                tcp_stream.peer_addr()?
                            );
                        }
                        Err(e) => panic!(),
                    }
                }
            }
        }
    }

    Ok(())
}
