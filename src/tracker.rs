use ratatui::{prelude::*, widgets::*};
use super::{Combatant, State};

/// Manages the initiative tracker.
#[derive(Default)]
pub struct Tracker {
    /// The current state / view of the tracker, indicating what actions the user can take at the
    /// current moment.
    state: State,

    /// The combatants of the encounter, ordered by initiative. The first combatant in the list is
    /// the combatant with the highest initiative.
    combatants: Vec<Combatant>,
}

impl Tracker {
    /// Creates a new initiative tracker.
    pub fn new() -> Tracker {
        Tracker::default()
    }

    /// Render the tracker to a [`Table`] widget.
    pub fn render(&self) -> Table {
        Table::new(
            vec![Row::new(["24", "Alice"]).height(2), Row::new(["2", "Bob"]).height(2)],
            vec![Constraint::Percentage(50), Constraint::Percentage(50)],
        )
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .padding(Padding::horizontal(1))
            )
            .header(
                Row::new([Text::from("Initiative").centered(), Text::from("Name").centered()])
                    .bold()
                    .height(2)
            )
    }
}
