pub struct StreamBuffer {
    pub incoming: Vec<u8>,
    pub outgoing: Vec<u8>,
}

impl Default for StreamBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamBuffer {
    pub fn new() -> Self {
        Self {
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }

    pub fn clear_incoming(&mut self) {
        self.incoming.clear();
    }

    pub fn clear_outgoing(&mut self) {
        self.outgoing.clear();
    }
}
