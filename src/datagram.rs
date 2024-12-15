use std::io::Write;
use std::io::Read;

pub enum Datagram {
    Config,
    Ack,
    FrameData,
}

#[derive(Debug)]
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub frame_rate: u16,
    pub target_bitrate: u32,
}

impl Config {
    pub fn serialize<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write(&self.width.to_be_bytes())?;
        writer.write(&self.height.to_be_bytes())?;
        writer.write(&self.frame_rate.to_be_bytes())?;
        writer.write(&self.target_bitrate.to_be_bytes())?;

        Ok(())
    }

    pub fn deserialize<R: Read>(mut reader: R) -> std::io::Result<Self> {
        let mut buf= [0; 10];
        reader.read_exact(&mut buf)?;

        Ok(Config {
            width: u16::from_be_bytes(buf[0..2].try_into().expect("hard coded slice is okay")),
            height: u16::from_be_bytes(buf[2..4].try_into().expect("hard coded slice is okay")),
            frame_rate: u16::from_be_bytes(buf[4..6].try_into().expect("hard coded slice is okay")),
            target_bitrate: u32::from_be_bytes(buf[6..10].try_into().expect("hard coded slice is okay")),
        })
    }
}