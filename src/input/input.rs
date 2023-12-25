use crate::config::action::Action;
use crate::config::config::Config;
use crate::data::data::Data;
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{event, ExecutableCommand};
use std::io::stdout;
use std::process::exit;

fn handle_key_inputs(key_event: KeyEvent, data: &mut Data, config: &mut Config) {
    let key = config
        .keybindings
        .iter()
        .find(|(keycode, _)| **keycode == key_event.code);

    if let Some((_, action)) = key {
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
            _ => {}
        }
    } else if let Char(char) = key_event.code {
        data.add_character(char);
    }
}

pub fn process_inputs(data: &mut Data, config: &mut Config) {
    if event::poll(std::time::Duration::from_millis(16)).expect("Could not poll inputs") {
        if let event::Event::Key(key) = event::read().expect("Could not read event") {
            if key.kind == KeyEventKind::Press {
                handle_key_inputs(key, data, config);
            }
        }
    }
}
