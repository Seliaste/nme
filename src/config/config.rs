use std::{collections::HashMap, fs, process::exit};

use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use toml::Value;

use super::action::Action;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub keybindings: HashMap<KeyCode, Action>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            keybindings: HashMap::new(),
        }
    }

    // TODO: implement this
    pub fn from(path: &str) -> Self {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => String::from(""),
        };
        let data: Value = match toml::from_str(&content) {
            Ok(d) => d,
            Err(_) => Value::Integer(0),
        };
        let keybindings = data
            .as_table()
            .unwrap()
            .get("keybindings")
            .unwrap()
            .as_table()
            .unwrap();
        let mut keybindings_map = HashMap::new();
        for (action, keycode) in keybindings.iter() {
            let action = Action::from(action.clone().to_owned());
            let keycode = string_to_keycode(keycode.as_str().unwrap());
            keybindings_map.insert(keycode, action);
        }
        Self {
            keybindings: keybindings_map,
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
