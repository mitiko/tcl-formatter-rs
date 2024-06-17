pub struct Formatter {
    buf: Vec<u8>,
}

impl Formatter {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { buf }
    }

    pub fn run(self) -> Vec<u8> {
        self.buf
    }
}
