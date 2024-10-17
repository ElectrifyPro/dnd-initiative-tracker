mod combatant;
pub mod state;

use combatant::Combatant;
use ratatui::{prelude::*, widgets::*};

/// Manages the initial state of the program, where the user can add combatants to the encounter.
#[derive(Default)]
pub struct EncounterBuilder {
    /// The combatants of the encounter, ordered by initiative. The first combatant in the list is
    /// the combatant with the highest initiative.
    combatants: Vec<Combatant>,

    /// The index of the currently highlighted combatant.
    highlighted: Option<usize>,
}

impl EncounterBuilder {
    /// Creates a new encounter builder.
    pub fn new() -> EncounterBuilder {
        EncounterBuilder::default()
    }

    /// Returns a reference to the combatants in the encounter builder.
    pub fn combatants(&self) -> &[Combatant] {
        &self.combatants
    }

    /// Adds a new combatant to the encounter builder.
    pub fn add_combatant(&mut self, combatant: Combatant) {
        self.combatants.push(combatant);
    }

    /// Gets reference to the combatant at the given index.
    pub fn combatant(&self, idx: usize) -> Option<&Combatant> {
        self.combatants.get(idx)
    }

    /// Gets a mutable reference to the combatant at the given index.
    pub fn combatant_mut(&mut self, idx: usize) -> Option<&mut Combatant> {
        self.combatants.get_mut(idx)
    }

    /// Highlight the combatant at the given index.
    pub fn highlight(&mut self, idx: usize) {
        self.highlighted = Some(idx);
    }

    /// Remove the highlight on the currently highlighted combatant.
    pub fn unhighlight(&mut self) {
        self.highlighted = None;
    }

    /// Render the encounter builder to a [`Table`] widget.
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
                Constraint::Fill(1),    // name
                Constraint::Length(14), // hp / max hp
                Constraint::Length(10), // temp hp
            ],
        )
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .padding(Padding::horizontal(1))
                    .title("Encounter Builder")
            )
            .header(
                Row::new([
                    Text::from("Name").centered(),
                    Text::from("HP / Max HP").centered(),
                    Text::from("Temp HP").centered(),
                ])
                    .bold()
                    .height(2)
            )
    }
}
