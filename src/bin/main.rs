use std::fs::File;
use nme::arguments;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result, Read};
use crate::arguments::parse_file_args;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let filename = parse_file_args().expect("Please specify a file to open");
    let mut text= String::new();
    let mut file = File::open(filename).expect("Could not open file");
    file.read_to_string(&mut text).expect("Could not read file");
    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            frame.render_widget(
                Paragraph::new(text.clone()),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}