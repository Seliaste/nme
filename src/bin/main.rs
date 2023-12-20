use std::fs::File;
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
};
use std::io::{stdout, Result, Read};
use nme::arguments::parse_file_args;
use nme::data::cursor::TextCursor;
use nme::data::data::Data;
use nme::input::input::process_inputs;
use nme::tui::main::display;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut data = Data { text: vec![], cursor: TextCursor { line: 0, character: 0 } };

    let filename = parse_file_args().expect("Please specify a file to open");
    let mut text = String::new();
    let mut file = File::open(filename).expect("Could not open file");
    file.read_to_string(&mut text).expect("Could not read file");
    data.text = text.split("\n").map(str::to_string).collect();

    loop {
        process_inputs(&mut data);
        display(&mut terminal, &data);
    }
}