use clap::{ArgEnum, Args};
use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::cli::ParseClientError;

#[derive(Debug, ArgEnum, Clone)]
pub enum ClientSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Args)]
pub struct Client {
    #[clap(long, short)]
    pub name: String,
    #[clap(long, short)]
    pub server_addr: Ipv4Addr,
    #[clap(long, short, arg_enum)]
    pub side: ClientSide,
}

impl FromStr for Client {
    type Err = ParseClientError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let client_conf: Vec<&str> = s.split(' ').collect();

        Ok(Self {
            name: String::from(client_conf[0]),
            server_addr: Ipv4Addr::new(0, 0, 0, 0),
            side: ClientSide::from_str(client_conf[1], true).unwrap(),
        })
    }
}
