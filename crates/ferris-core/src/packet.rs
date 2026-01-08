#[derive(Debug, Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    pub pts: Option<i64>,
    pub dts: Option<i64>,
    pub duration: Option<i64>,
    pub stream_index: usize,
}

impl Packet {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            pts: None,
            dts: None,
            duration: None,
            stream_index: 0,
        }
    }
}
