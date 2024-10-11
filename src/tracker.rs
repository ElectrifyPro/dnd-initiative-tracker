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

    /// The index of the currently highlighted combatant.
    highlighted: Option<usize>,
}

impl Tracker {
    /// Creates a new initiative tracker.
    pub fn new() -> Tracker {
        Tracker::default()
    }

    /// Returns a reference to the combatants in the tracker.
    pub fn combatants(&self) -> &[Combatant] {
        &self.combatants
    }

    /// Adds a new combatant to the initiative tracker.
    pub fn add_combatant(&mut self, combatant: Combatant) {
        self.combatants.push(combatant);

        // highest initiative first
        self.sort();
    }

    /// Gets reference to the combatant at the given index.
    pub fn combatant(&self, idx: usize) -> Option<&Combatant> {
        self.combatants.get(idx)
    }

    /// Gets a mutable reference to the combatant at the given index.
    pub fn combatant_mut(&mut self, idx: usize) -> Option<&mut Combatant> {
        self.combatants.get_mut(idx)
    }

    /// Sorts the combatants by initiative.
    pub fn sort(&mut self) {
        self.combatants.sort_by(|a, b| b.initiative().cmp(&a.initiative()));
    }

    /// Highlight the combatant at the given index.
    pub fn highlight(&mut self, idx: usize) {
        self.highlighted = Some(idx);
    }

    /// Remove the highlight on the currently highlighted combatant.
    pub fn unhighlight(&mut self) {
        self.highlighted = None;
    }

    /// Render the tracker to a [`Table`] widget.
    pub fn render(&self) -> Table {
        Table::new(
            self.combatants.iter()
                .enumerate()
                .map(|(i, combatant)| {
                    let row = combatant.row().height(2);
                    if Some(i) == self.highlighted {
                        row.style(Style::default().bg(Color::Rgb(0, 48, 130)))
                    } else {
                        row
                    }
                }),
            [
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
