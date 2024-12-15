use core::str;
use std::net::UdpSocket;

use clap::Parser;

#[derive(Parser)]
struct Args {
    address: String
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let socket = UdpSocket::bind(args.address)?;

    let mut buf = [0; 20];
    socket.recv_from(&mut buf)?;
    print!("{:?}\n", str::from_utf8(&buf).expect("why"));
    Ok(())
}
