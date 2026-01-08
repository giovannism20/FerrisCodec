use ferris_core::{Packet, Result};
use ferris_io::MediaSink;

pub trait Muxer {
    fn open(&mut self, sink: &mut dyn MediaSink) -> Result<()>;
    fn write_packet(&mut self, packet: &Packet) -> Result<()>;
    fn finalize(&mut self) -> Result<()>;
}
