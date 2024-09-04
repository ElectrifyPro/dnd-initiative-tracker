use crate::{state::State, tracker::Tracker};
use crossterm::{
    event,
    terminal,
    execute,
};
use ratatui::{prelude::*, widgets::*};
use std::io;

/// The locations of all widgets on the screen.
#[derive(Debug, Clone, Copy)]
pub struct RenderLocations {
    /// The table showing all combatants, their initiative, statuses, conditions, etc. This appears
    /// at the top of the screen.
    pub combatant_table: Rect,

    /// The box showing the available commands for the current state. This appears at the
    /// bottom-right.
    pub guide: Rect,

    /// The box used by the current state. This appears at the bottom-left.
    pub state: Rect,
}

/// Terminal handler.
pub struct Ui {
    /// The terminal.
    terminal: Terminal<CrosstermBackend<io::Stdout>>,

    /// The locations of all widgets on the screen.
    locations: RenderLocations,
}

impl Ui {
    /// Initializes the terminal.
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;

        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        let size = terminal.size()?;
        Ok(Self {
            terminal,
            locations: {
                let main_layout = Layout::vertical([
                    Constraint::Percentage(75),
                    Constraint::Percentage(25),
                ])
                    .horizontal_margin(1)
                    .vertical_margin(1)
                    .split(size);
                let state_layout = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                    .split(main_layout[1]);
                RenderLocations {
                    combatant_table: main_layout[0],
                    guide: state_layout[0],
                    state: state_layout[1],
                }
            },
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

    /// Renders the initiative tracker to the terminal.
    pub fn render(&mut self, tracker: &Tracker, state: &State) -> io::Result<()> {
        self.terminal.draw(|f| {
            f.render_widget(tracker.render(), self.locations.combatant_table);
            f.render_widget(
                Paragraph::new(state.default_help())
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(Color::White))
                            .padding(Padding::horizontal(1))
                            .title("Help"),
                    ),
                self.locations.guide,
            );
            if let Some(widget) = state.render() {
                f.render_widget(widget, self.locations.state);
            } else {
                f.render_widget(
                    Paragraph::new(Text::from("no state is active"))
                        .block(
                            Block::default()
                                .padding(Padding::top(self.locations.state.height / 2)) // vertical padding
                        )
                        .centered(), // horizontal align
                    self.locations.state,
                );
            }
        })?;
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
