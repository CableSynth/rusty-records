use std::io::{Read, Write, stdout, stdin, Error};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::input;
use termion::screen::AlternateScreen;
use termion::event::Key;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), Error> {
    let stdout = stdout().into_raw_mode()?;
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(7),
                    Constraint::Percentage(85),
                    Constraint::Percentage(8)
                ].as_ref()
            )
            .split(frame.size());
        let search = Block::default()
             .title("Search")
             .borders(Borders::ALL);
        frame.render_widget(search, chunks[0]);
        let songs = Block::default()
             .title("Block 2")
             .borders(Borders::ALL);
        frame.render_widget(songs, chunks[1]);

    })?;
    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap();

        match b.unwrap() {
            b'q' => break Ok(()),
            a => (),
        };
        // backend.flush();
    }
}
