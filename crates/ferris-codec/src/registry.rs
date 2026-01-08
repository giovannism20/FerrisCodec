#[derive(Debug, Clone)]
pub struct CodecDescriptor {
    pub name: &'static str,
    pub long_name: &'static str,
    pub mime_types: &'static [&'static str],
}

#[derive(Default)]
pub struct CodecRegistry {
    codecs: Vec<CodecDescriptor>,
}

impl CodecRegistry {
    pub fn new() -> Self {
        Self { codecs: Vec::new() }
    }

    pub fn register(&mut self, codec: CodecDescriptor) {
        self.codecs.push(codec);
    }

    pub fn all(&self) -> &[CodecDescriptor] {
        &self.codecs
    }
}
