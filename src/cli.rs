use crate::client::Client;
use clap::Parser;
use std::error::Error;
use std::fmt;

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

/// Share your mouse across computers through network
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, short)]
    pub server_addr: Option<String>,
    #[clap(long, short, required_unless_present = "server-addr")]
    pub client: Option<Client>,
}
