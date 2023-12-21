use crate::config::action::Action;
use crate::config::config::Config;
use crate::data::data::Data;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{event, ExecutableCommand};
use std::io::stdout;
use std::process::exit;

struct Controller;

impl Controller {
    fn handle_key_inputs(key_event: KeyEvent, data: &mut Data, config: &mut Config) {
        let key = config
            .keymaps
            .iter()
            .find(|(_name, key)| **key == key_event.code);

        if let Some((action, _)) = key {
            match action {
                Action::DeleteLine => {}
                Action::SaveFile => data.save(),
                Action::MoveUp => data.move_up(),
                Action::MoveDown => data.move_down(),
                Action::MoveLeft => data.move_left(),
                Action::MoveRight => data.move_right(),
                Action::DeleteChar => data.remove_character(),
                Action::AddLine => data.add_line(),
                Action::Exit => {
                    stdout()
                        .execute(LeaveAlternateScreen)
                        .expect("Could not leave alternate screen");
                    disable_raw_mode().expect("Could not disable raw mode");
                    exit(0);
                }
            }
        } else {
            match key_event.code {
                Char(char) => data.add_character(char),
                // not a keybinding and not a character
                _ => {}
            }
        }
    }
}

pub fn process_inputs(data: &mut Data, config: &mut Config) {
    if event::poll(std::time::Duration::from_millis(16)).expect("Could not poll inputs") {
        if let event::Event::Key(key) = event::read().expect("Could not read event") {
            if key.kind == KeyEventKind::Press {
                Controller::handle_key_inputs(key, data, config);
            }
        }
    }
}
