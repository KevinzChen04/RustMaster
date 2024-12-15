use serde::{Deserialize, Serialize};

pub enum Datagram {
    Config,
    Ack,
    FrameData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub frame_rate: u16,
    pub target_bitrate: u32,
}