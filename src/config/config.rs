use std::{collections::HashMap, fs};

use crossterm::event::KeyCode;
use serde::Deserialize;
use toml::Value;

use super::action::Action;

#[derive(Deserialize, Debug)]
struct Data {
    pub keybindings: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct ConfigFile {
    // pub keybindings: HashMap<String, Value>,
    // pub deletechar: String,
}

pub struct Config {
    pub keymaps: HashMap<Action, KeyCode>,
}

impl Config {
    pub fn testing() -> Self {
        let mut k = HashMap::new();
        // Testing Defaults
        k.insert(Action::DeleteLine, KeyCode::Char('d'));
        k.insert(Action::Exit, KeyCode::Esc);
        k.insert(Action::MoveUp, KeyCode::Up);
        k.insert(Action::MoveDown, KeyCode::Down);
        k.insert(Action::MoveLeft, KeyCode::Left);
        k.insert(Action::MoveRight, KeyCode::Right);
        k.insert(Action::AddLine, KeyCode::Enter);
        k.insert(Action::DeleteChar, KeyCode::Backspace);
        k.insert(Action::SaveFile, KeyCode::F(1));
        Self { keymaps: k }
    }

    pub fn new() -> Self {
        Self {
            keymaps: HashMap::new(),
        }
    }

    // TODO: implement this
    pub fn from(path: &str) -> Self {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => String::from(""),
        };
        let data: Data = match toml::from_str(&content) {
            Ok(d) => d,
            Err(_) => Data {
                keybindings: HashMap::new(),
            },
        };
        let mut keybindings = HashMap::new();
        for (shortcut, keymap) in data.keybindings.iter() {
            let action: Action = (*shortcut).clone().into();
            let keycode_str = keymap.as_str().unwrap().to_string();
            let keycode = string_to_keycode(&keycode_str);
            keybindings.insert(action, keycode);
        }
        Self {
            keymaps: keybindings,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

//TODO: too tedious, find another way (maybe with serde?)
fn string_to_keycode(value: &str) -> KeyCode {
    match value.to_lowercase().as_str() {
        "backspace" => KeyCode::Backspace,
        "f1" => KeyCode::F(1),
        "enter" => KeyCode::Enter,
        "down" => KeyCode::Down,
        "up" => KeyCode::Up,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "escape" => KeyCode::Esc,
        _ => KeyCode::Null,
    }
}
