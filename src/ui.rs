use crate::view::{State, View};
use crossterm::{
    cursor,
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
    /// bottom-left.
    pub guide: Rect,

    /// The box used by the current state. This appears at the bottom-right.
    pub state: Rect,

    /// The box used for user input. This appears at the bottom-right, below the state box.
    pub input: Rect,
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
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            event::PushKeyboardEnhancementFlags(event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES),
        )?;

        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        // hide real cursor
        terminal.hide_cursor()?;
        let size = terminal.size()?;
        Ok(Self {
            terminal,
            locations: {
                let full_layout = Layout::vertical([
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
                    .split(full_layout[1]);
                let input_layout = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(3),
                ])
                    .split(state_layout[1]);
                RenderLocations {
                    combatant_table: full_layout[0],
                    guide: state_layout[0],
                    state: input_layout[0],
                    input: input_layout[1],
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

    /// Renders the view to the terminal.
    pub fn render_view<S, V>(&mut self, view: &V, state: &S) -> io::Result<()>
    where
        S: State,
        V: View<State = S>,
    {
        self.terminal.draw(|f| {
            f.render_widget(view.render(), self.locations.combatant_table);
            f.render_widget(
                Paragraph::new(state.help())
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(Color::White))
                            .padding(Padding::horizontal(1))
                            .title("Help"),
                    ),
                self.locations.guide,
            );

            if let Some((widget, input)) = state.render() {
                f.render_widget(widget, self.locations.state);
                if let Some(input) = input {
                    f.render_widget(input, self.locations.input);
                }
            } else {
                f.render_widget(
                    Paragraph::new(Text::from("no state is active"))
                        .block(
                            Block::default()
                                .padding(Padding::top(self.locations.state.height / 2)) // vertical padding
                        )
                        .centered(), // horizontal align
                    self.locations.state.union(self.locations.input),
                );
            }
        })?;
        Ok(())
    }
}

/// Cleans up the terminal.
impl Drop for Ui {
    fn drop(&mut self) {
        self.terminal.show_cursor().unwrap();
        execute!(
            io::stdout(),
            event::PopKeyboardEnhancementFlags,
            terminal::LeaveAlternateScreen,
        ).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
