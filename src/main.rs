use clap::{ArgEnum, Args, Parser};
use local_ip_address::local_ip;
use std::error::Error;
use std::fmt;
use std::net::TcpListener;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseClientSideError;

impl fmt::Display for ParseClientSideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseClientSideError")
    }
}

impl Error for ParseClientSideError {}

#[derive(Debug, Clone)]
pub struct ParseClientError;

impl fmt::Display for ParseClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseClientError")
    }
}

impl Error for ParseClientError {}

#[derive(Debug, ArgEnum, Clone)]
pub enum ClientSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Args)]
pub struct Client {
    #[clap(long, short)]
    name: String,
    #[clap(long, short, arg_enum)]
    position: ClientSide,
}

impl FromStr for Client {
    type Err = ParseClientError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let client_conf: Vec<&str> = s.split(' ').collect();

        Ok(Self {
            name: String::from(client_conf[0]),
            position: ClientSide::from_str(client_conf[1], true).unwrap(),
        })
    }
}

/// Share your mouse across computers through network
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, short)]
    server_ip: Option<String>,
    #[clap(long, short, required_unless_present = "server-ip")]
    client: Option<Client>,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.server_ip {
        Some(ip) => {}
        None => {
            let local_ip = local_ip().unwrap();

            let client = args.client.unwrap();

            println!("Runnig hijack server on: {:?}", local_ip);
            println!(
                "Client: {} configured at {:?} position.",
                client.name, client.position
            );
            {
                let listener = TcpListener::bind("0.0.0.0:4242")?;

                for stream in listener.incoming() {
                    println!("aaaaaa");
                    println!("{:?}", stream);
                }
            }
        }
    }

    Ok(())
}
