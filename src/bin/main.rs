use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use nme::arguments::parse_file_args;
use nme::config::config::Config;
use nme::data::data::Data;
use nme::input::input::process_inputs;
use nme::tui::main::display;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::fs::{File, OpenOptions};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let filename = parse_file_args().expect("Please specify a file to open");
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(filename)
        .expect("Could not open file");
    let mut data = Data::new(file);
    // TODO: change this when done testing
    let mut config = Config::testing();

    loop {
        process_inputs(&mut data, &mut config);
        display(&mut terminal, &data);
    }
}
