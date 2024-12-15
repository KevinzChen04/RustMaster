use std::net::UdpSocket;
use rust_master::datagram::Config;
use clap::Parser;

#[derive(Parser)]
struct Args {
    address: String
}

fn main() -> std::io::Result<()> {
    let local_host = "127.0.0.1:0";
    let args = Args::parse();
    let socket = UdpSocket::bind(local_host)?;

    let send_this = Config {
        width: 1,
        height: 2,
        frame_rate: 3,
        target_bitrate: 4,
    };

    let mut writer = Vec::new();
    send_this.serialize(&mut writer)?;
    socket.send_to(&writer, args.address)?;
    Ok(())
}
