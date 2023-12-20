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
}