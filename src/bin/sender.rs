use std::net::UdpSocket;
use clap::Parser;

#[derive(Parser)]
struct Args {
    address: String
}

fn main() -> std::io::Result<()> {
    let local_host = "127.0.0.1:0";
    let args = Args::parse();
    let socket = UdpSocket::bind(local_host)?;

    let send_this = "Hello World!";
    let src = args.address;
    socket.send_to(send_this.as_bytes(), src)?;
    Ok(())
}
