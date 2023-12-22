use crate::data::cursor::TextCursor;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::usize;

pub struct Data {
    pub file: File,
    pub text: Vec<String>,
    pub cursor: TextCursor,
    has_tmp_cursor: bool,
    tmp_cursor_character: u16, // to keep max in case of a line switch
}

impl Data {
    pub fn new(mut file: File) -> Data {
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Could not read file");
        Data {
            file,
            text: text.split('\n').map(str::to_string).collect(),
            cursor: TextCursor {
                line: 0,
                character: 0,
            },
            has_tmp_cursor: false,
            tmp_cursor_character: 0,
        }
    }

    fn get_text_after_cusor(&self) -> &str {
        let res = &self.text.get(self.cursor.line as usize).unwrap();
        &res[self.cursor.character as usize..]
    }

    pub fn add_line(&mut self) {
        self.has_tmp_cursor = false;
        let to_newline = self.get_text_after_cusor();

        self.text
            .insert(self.cursor.line as usize + 1, to_newline.to_string());
        let to_del = self.text.get(self.cursor.line as usize).unwrap();
        let to_del = to_del
            .chars()
            .take(self.cursor.character as usize)
            .collect();
        self.text[self.cursor.line as usize] = to_del;
        self.cursor.line += 1;
        self.cursor.character = 0;
        self.check_valid_cursor_position();
    }

    pub fn del_line(&mut self) {
        self.has_tmp_cursor = false;
        let to_be_brought_up = self.get_text_after_cusor();
        let mut new_line = self
            .text
            .get(self.cursor.line as usize - 1)
            .unwrap()
            .to_owned();
        let old_line_size = new_line.len();
        new_line.push_str(to_be_brought_up);
        self.text[self.cursor.line as usize - 1] = new_line;
        self.text.remove(self.cursor.line as usize);
        self.cursor.line -= 1;
        self.cursor.character = old_line_size as u16;
    }

    pub fn add_character(&mut self, character: char) {
        self.has_tmp_cursor = false;
        if self.cursor.line > self.text.len() as u16 {
            self.text.insert(self.cursor.line as usize, "".to_string())
        }
        let mut line: String = self.text[self.cursor.line as usize].to_string();
        line.insert(self.cursor.character as usize, character);
        self.text[self.cursor.line as usize] = line;
        self.cursor.character += 1;
    }

    pub fn remove_character(&mut self) {
        if self.cursor.character == 0 {
            if self.cursor.line == 0 {
                return;
            }
            self.del_line();
            return;
        }
        let mut line: String = self.text[self.cursor.line as usize].to_string();
        line.remove(self.cursor.character as usize - 1);
        self.text[self.cursor.line as usize] = line;
        self.cursor.character -= 1;
    }

    fn check_valid_cursor_position(&mut self) {
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
        if self.cursor.line + 1 < self.text.len() as u16 {
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
        if self.cursor.character != 0 {
            self.cursor.character -= 1;
        }
    }

    pub fn save(&mut self) {
        self.file
            .set_len(0)
            .expect("Could not truncate file before writing");
        self.file.rewind().expect("Could not rewind file to save");
        self.file
            .write_all(self.text.join("\n").as_bytes())
            .expect("Could not save file");
    }
}
