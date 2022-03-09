use local_ip_address::local_ip;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let my_local_ip = local_ip().unwrap();

    println!("{:?}", my_local_ip);
    {
        let listener = TcpListener::bind("0.0.0.0:4242")?;

        for stream in listener.incoming() {
            println!("aaaaaa");
            println!("{:?}", stream);
        }
    }

    Ok(())
}
