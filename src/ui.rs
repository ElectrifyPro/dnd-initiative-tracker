use crossterm::{
    event,
    terminal,
    execute,
};
use ratatui::{prelude::*, widgets::*};
use std::io;

/// Terminal handler.
pub struct Ui {
    /// The terminal.
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Ui {
    /// Initializes the terminal.
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout()))?,
        })
    }

    /// Reads a character from the terminal.
    pub fn read_char(&self) -> io::Result<char> {
        loop {
            match event::read()? {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Char(c) => break Ok(c),
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    /// Renders the given widget to the terminal.
    pub fn render<W>(&mut self, widget: W) -> io::Result<()>
    where
        W: Widget,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref());
        self.terminal.draw(|f| f.render_widget(widget, f.size()))?;
        Ok(())
    }
}

/// Cleans up the terminal.
impl Drop for Ui {
    fn drop(&mut self) {
        execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
