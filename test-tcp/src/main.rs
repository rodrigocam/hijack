use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.0.109:4242")?;

    stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here
