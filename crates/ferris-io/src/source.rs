use ferris_core::Result;

pub trait MediaSource {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}
