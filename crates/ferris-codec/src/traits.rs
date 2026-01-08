use ferris_core::{Frame, Packet, Result};

pub trait Decoder {
    fn send_packet(&mut self, packet: &Packet) -> Result<()>;
    fn receive_frame(&mut self) -> Result<Option<Frame>>;
}

pub trait Encoder {
    fn send_frame(&mut self, frame: &Frame) -> Result<()>;
    fn receive_packet(&mut self) -> Result<Option<Packet>>;
}
