use ferris_core::Result;

#[derive(Debug, Clone, Copy)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait MediaSeek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}
