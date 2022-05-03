use hijack_lib::server::Server;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    // let args = Cli::parse();

    let server = Server::new();
    server.run();

    Ok(())
}
