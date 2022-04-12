use clap::{Args, Parser};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    clients: Vec<ClientConfig>,
}

#[derive(Debug, Clone, Deserialize, Args)]
pub struct ClientConfig {
    name: String,
    side: ClientSide,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ClientSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct ParseClientSideError;

impl FromStr for ClientSide {
    type Err = ParseClientSideError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(ParseClientSideError),
        }
    }
}

impl fmt::Display for ParseClientSideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseClientSideError")
    }
}

impl Error for ParseClientSideError {}

/// Share your mouse across computers through network
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, short)]
    pub config: Option<String>,
    #[clap(long, short, required_unless_present = "config")]
    pub server_addr: Option<String>,
    #[clap(long, short, requires = "server-addr")]
    pub name: Option<String>,
}

pub fn exec_server(config_str: String) {}
