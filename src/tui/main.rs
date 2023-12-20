use std::io::Stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::Paragraph;
use crate::data::data::Data;

pub fn display(terminal: &mut Terminal<CrosstermBackend<Stdout>>, data: &Data) {
    terminal.draw(|frame| {
        let area = frame.size();

        frame.render_widget(
            Paragraph::new(data.text.to_owned().join("\n")),
            area,
        );
    }).expect("Frame could not be drawn");
    terminal.set_cursor(data.cursor.character, data.cursor.line).expect("Could not move cursor");
    terminal.show_cursor().expect("Unable to show cursor");
}