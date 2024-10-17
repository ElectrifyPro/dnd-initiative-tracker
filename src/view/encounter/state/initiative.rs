use crate::{input::Input, view::encounter::{state::State, EncounterBuilder}};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};

/// Roll initiative to start the combat encounter.
#[derive(Debug, PartialEq, Eq)]
pub struct RollInitiative {
    /// The name of the combatant we're rolling initiative for.
    pub name: Option<String>,

    /// The initiative roll of the combatant.
    pub initiative: Option<i32>,

    /// The index of the currently focused combatant.
    pub row: usize,

    pub input: Input,
}

impl Default for RollInitiative {
    fn default() -> Self {
        Self {
            name: None,
            initiative: None,
            row: 0,
            input: Input::default()
                .with_ignore([
                    KeyCode::Char('+'),
                    KeyCode::Char('='),
                    KeyCode::Char('-'),
                    KeyCode::Char('_'),
                ].into()),
        }
    }
}

impl RollInitiative {
    /// Returns the [`Input`] widget.
    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn help(&self) -> String {
        "<escape>: cancel, back to encounter builder
<enter>: set initiative
<ctrl-enter>: set initiative and finish, sort all combatants
+ or =: previous combatant
- or _: next combatant".to_string()
    }

    pub fn render(&self) -> Table {
        Table::new(
            [
                Row::new([Text::from("Initiative").bold(), Text::from(self.input.as_str())])
                    .style(Style::default().bg(Color::Rgb(0, 48, 130)))
            ],
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .padding(Padding::horizontal(1))
                    .title(format!("Rolling Initiative for {}", self.name.as_deref().unwrap_or_else(|| "Unknown")))
            )
    }

    pub fn set_row(&mut self, row: usize, encounter_builder: &mut EncounterBuilder) {
        self.row = row;
        encounter_builder.highlight(row);
        self.name = encounter_builder.combatant(row)
            .map(|c| c.name.clone())
            .flatten();

        if let Some(combatant) = encounter_builder.combatant(row) {
            self.name = combatant.name.clone();
            self.initiative = combatant.initiative.clone();
            if let Some(initiative) = self.initiative.as_ref() {
                self.input.set(initiative.to_string());
            }
        }
    }

    pub fn init_encounter(&mut self, encounter_builder: &mut EncounterBuilder) {
        self.set_row(0, encounter_builder);
    }

    pub fn handle_event(&mut self, key: KeyEvent, encounter_builder: &mut EncounterBuilder) -> Option<State> {
        let Some(unhandled_key) = self.input.update(key) else {
            return None;
        };
        match unhandled_key.code {
            KeyCode::Esc => Some(State::Home),
            KeyCode::Enter => {
                let initiative = Some(self.input.take().parse().ok().unwrap_or_default());
                encounter_builder.combatant_mut(self.row).map(|c| c.initiative = initiative);

                let next_row = (self.row + 1) % encounter_builder.combatants().len();
                self.set_row(next_row, encounter_builder);
                None
            },
            KeyCode::Char('+') | KeyCode::Char('=') => {
                let next_row = (self.row + 1) % encounter_builder.combatants().len();
                self.set_row(next_row, encounter_builder);
                None
            },
            KeyCode::Char('-') | KeyCode::Char('_') => {
                let prev_row = self.row.wrapping_sub(1).min(encounter_builder.combatants().len() - 1);
                self.set_row(prev_row, encounter_builder);
                None
            },
            // enter seems to return ctrl-j
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let initiative = self.input.take().parse().ok().unwrap_or_default();
                encounter_builder.combatant_mut(self.row).map(|c| c.initiative = Some(initiative));

                // tracker.sort();

                Some(State::Home)
            },
            _ => None,
        }
    }
}
