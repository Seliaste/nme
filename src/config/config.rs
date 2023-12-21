use std::collections::HashMap;

use crossterm::event::KeyCode;

use super::action::Action;

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
    pub fn from() -> Self {
        Self {
            keymaps: HashMap::new(),
        }
    }
}
