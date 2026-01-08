use ferris_core::{Frame, Result};

pub trait FilterNode {
    fn process(&mut self, frame: Frame) -> Result<Frame>;
}

pub struct FilterGraph {
    nodes: Vec<Box<dyn FilterNode>>,
}

impl FilterGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Box<dyn FilterNode>) {
        self.nodes.push(node);
    }

    pub fn process(&mut self, mut frame: Frame) -> Result<Frame> {
        for node in &mut self.nodes {
            frame = node.process(frame)?;
        }
        Ok(frame)
    }
}
