#[derive(Debug, Clone)]
pub struct Frame {
    pub data: Vec<u8>,
    pub pts: Option<i64>,
}

impl Frame {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, pts: None }
    }
}
