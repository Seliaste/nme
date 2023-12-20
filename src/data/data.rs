use crate::data::cursor::TextCursor;


pub struct Data {
    pub text: Vec<String>,
    pub cursor: TextCursor,
}

impl Data {
    pub fn add_character(&mut self, character: char){
        if self.cursor.line > self.text.len() as u16 {
            self.text.insert(self.cursor.line as usize, "".to_string())
        }
        let mut line: String = self.text[self.cursor.line.clone() as usize].to_string();
        line.insert(self.cursor.character as usize, character);
        self.text[self.cursor.line.clone() as usize] = line;
        self.cursor.character+=1;
    }

    fn check_valid_cursor_position(&mut self){
        // if cursor is further than end of new line, move it at the end of it
        if self.cursor.character >= self.text[self.cursor.line as usize].len() as u16 {
            self.cursor.character = self.text[self.cursor.line as usize].len() as u16
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.line != 0 {
            self.cursor.line -= 1;
            self.check_valid_cursor_position();
        }
    }
    pub fn move_down(&mut self) {
        if self.cursor.line + 1 < self.text.len() as u16
        {
            self.cursor.line += 1;
            self.check_valid_cursor_position();
        };
    }
    pub fn move_right(&mut self) {
        if self.cursor.character < self.text[self.cursor.line as usize].len() as u16 {
            self.cursor.character += 1;
        }
    }
    pub fn move_left(&mut self) {
        if self.cursor.character != 0 {self.cursor.character -= 1;}
    }
}