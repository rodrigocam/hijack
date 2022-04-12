use clap::Parser;
use std::error;
use std::fs::File;
use std::io::Read;
use std::net::IpAddr;
use std::str::FromStr;

pub mod cli;
pub mod core;
pub mod observer;

use crate::cli::{Cli, ServerConfig};
use crate::core::{Client, Server};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Cli::parse();

    // if it has a config it is certain a hijack server
    match args.config {
        Some(path) => {
            let mut config_file = File::open(path)?;
            let mut config_str = String::new();
            config_file.read_to_string(&mut config_str)?;
            let config: ServerConfig = toml::from_str(&config_str)?;

            let server = Server::from_config(config);
            server.run();
        }
        // if a config is not present it is a client
        None => {
            let client = Client::new(
                args.name.unwrap(),
                IpAddr::from_str(&args.server_addr.unwrap())?,
            );
            client.run();
        }
    }
    Ok(())
}
