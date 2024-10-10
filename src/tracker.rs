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

    /// Adds a new combatant to the initiative tracker.
    pub fn add_combatant(&mut self, combatant: Combatant) {
        self.combatants.push(combatant);

        // highest initiative first
        self.combatants.sort_by(|a, b| b.initiative().cmp(&a.initiative()));
    }

    /// Render the tracker to a [`Table`] widget.
    pub fn render(&self) -> Table {
        Table::new(
            self.combatants.iter()
                .map(|combatant| combatant.row().height(2))
                .collect::<Vec<_>>(),
            vec![
                Constraint::Length(12), // initiative
                Constraint::Fill(1),    // name
                Constraint::Length(10), // actions
                Constraint::Length(14), // hp / max hp
                Constraint::Length(10), // temp hp
            ],
        )
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .padding(Padding::horizontal(1))
                    .title("Initiative Tracker")
            )
            .header(
                Row::new([
                    Text::from("Initiative").centered(),
                    Text::from("Name").centered(),
                    Text::from("Actions").centered(),
                    Text::from("HP / Max HP").centered(),
                    Text::from("Temp HP").centered(),
                ])
                    .bold()
                    .height(2)
            )
    }
}
