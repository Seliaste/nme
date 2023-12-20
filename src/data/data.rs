use std::fs::File;
use std::io::{Read, Seek, Write};
use crate::data::cursor::TextCursor;


pub struct Data {
    pub file: File,
    pub text: Vec<String>,
    pub cursor: TextCursor,
    has_tmp_cursor: bool,
    tmp_cursor_character: u16 // to keep max in case of a line switch
}

impl Data {
    pub fn new(mut file: File) -> Data{
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Could not read file");
        Data{file,
            text:text.split("\n").map(str::to_string).collect(),
            cursor:TextCursor { line: 0, character: 0 },
            has_tmp_cursor:false,tmp_cursor_character:0}
    }
    pub fn add_character(&mut self, character: char){
        self.has_tmp_cursor = false;
        if self.cursor.line > self.text.len() as u16 {
            self.text.insert(self.cursor.line as usize, "".to_string())
        }
        let mut line: String = self.text[self.cursor.line.clone() as usize].to_string();
        line.insert(self.cursor.character as usize, character);
        self.text[self.cursor.line.clone() as usize] = line;
        self.cursor.character+=1;
    }

    pub fn remove_character(&mut self){
        if self.cursor.character == 0 { return; }
        let mut line: String = self.text[self.cursor.line.clone() as usize].to_string();
        line.remove(self.cursor.character as usize-1);
        self.text[self.cursor.line.clone() as usize] = line;
        self.cursor.character-=1;
    }

    fn check_valid_cursor_position(&mut self){
        // if cursor is further than end of new line, move it at the end of it
        if self.cursor.character >= self.text[self.cursor.line as usize].len() as u16 {
            self.has_tmp_cursor = true;
            self.tmp_cursor_character = self.cursor.character;
            self.cursor.character = self.text[self.cursor.line as usize].len() as u16
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.line != 0 {
            self.cursor.line -= 1;
            if self.has_tmp_cursor {
                self.cursor.character = self.tmp_cursor_character;
            }
            self.check_valid_cursor_position();
        }
    }
    pub fn move_down(&mut self) {
        if self.cursor.line + 1 < self.text.len() as u16
        {
            self.cursor.line += 1;
            if self.has_tmp_cursor {
                self.cursor.character = self.tmp_cursor_character;
            }
            self.check_valid_cursor_position();
        };
    }
    pub fn move_right(&mut self) {
        self.has_tmp_cursor = false;
        if self.cursor.character < self.text[self.cursor.line as usize].len() as u16 {
            self.cursor.character += 1;
        }
    }
    pub fn move_left(&mut self) {
        self.has_tmp_cursor = false;
        if self.cursor.character != 0 {self.cursor.character -= 1;}
    }

    pub fn save(&mut self) {
        self.file.set_len(0).expect("Could not truncate file before writing");
        self.file.rewind().expect("Could not rewind file to save");
        self.file.write(self.text.join("\n").as_bytes()).expect("Could not save file");
    }
}