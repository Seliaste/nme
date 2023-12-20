pub struct TextCursor {
    pub line: u16,
    pub character: u16,
}

impl TextCursor {
    pub fn move_up(&mut self) {
        if self.line != 0 { self.line -= 1;}
    }
    pub fn move_down(&mut self) {
        self.line += 1;
    }
    pub fn move_right(&mut self) {
        self.character += 1;
    }
    pub fn move_left(&mut self) {
        if self.character != 0 {self.character -= 1;}
    }
}