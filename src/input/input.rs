use std::io::stdout;
use std::process::exit;
use crossterm::{event, ExecutableCommand};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use crossterm::event::KeyCode::Char;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crate::data::data::Data;

pub fn process_inputs(data: &mut Data) {
    if event::poll(std::time::Duration::from_millis(16)).expect("Could not poll inputs") {
        if let event::Event::Key(key) = event::read().expect("Could not read event") {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        stdout().execute(LeaveAlternateScreen).expect("Could not leave alternate screen");
                        disable_raw_mode().expect("Could not disable raw mode");
                        exit(0);
                    }
                    KeyCode::Left => data.move_left(),
                    KeyCode::Right => data.move_right(),
                    KeyCode::Up => data.move_up(),
                    KeyCode::Down => data.move_down(),
                    KeyCode::Backspace => data.remove_character(),
                    KeyCode::F(1) => data.save(),
                    Char(character) => {data.add_character(character)}
                    _ => {}
                }
            }
        }
    }
}