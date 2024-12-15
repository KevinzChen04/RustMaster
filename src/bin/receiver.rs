use std::net::UdpSocket;
use rust_master::datagram::Config;
use clap::Parser;

#[derive(Parser)]
struct Args {
    address: String
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let socket = UdpSocket::bind(args.address)?;

    let mut buf = [0; 494994];
    socket.recv_from(&mut buf)?;
    let config: Config = bincode::deserialize(&buf).expect("Incorrect config format");
    println!("{:?}", &config);
    Ok(())
}
