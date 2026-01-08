use ferris_core::{Packet, Result};
use ferris_io::MediaSource;

pub trait Demuxer {
    fn open(&mut self, source: &mut dyn MediaSource) -> Result<()>;
    fn read_packet(&mut self) -> Result<Option<Packet>>;
}
