use ferris_core::Result;
use ferris_io::MediaSource;

#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub format_name: &'static str,
    pub confidence: u8,
}

pub trait Prober {
    fn probe(&self, source: &mut dyn MediaSource) -> Result<Option<ProbeResult>>;
}
