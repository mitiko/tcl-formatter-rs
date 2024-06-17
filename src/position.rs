#[derive(Default)]
pub struct Position {
    line: usize,
    column: usize,
    idx: usize,
}

impl Position {
    pub fn move_char(&mut self) {
        self.idx += 1;
        self.column += 1;
    }
    pub fn move_line(&mut self) {
        self.idx += 1;
        self.line += 1;
        self.column = 0;
    }

    pub fn move_sym(&mut self, sym: u8) {
        if sym == b'\n' {
            self.move_line();
        } else {
            self.move_char();
        }
    }
}
