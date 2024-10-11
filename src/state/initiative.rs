use crate::{combatant::Combatant, input::Input, state::State, tracker::Tracker};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};

/// Roll initiative for a combat encounter.
#[derive(Default, PartialEq, Eq)]
pub struct RollInitiative {
    /// The name of the combatant we're rolling initiative for.
    pub name: Option<String>,

    /// The initiative roll of the combatant.
    pub initiative: Option<i32>,

    /// The index of the currently focused combatant.
    pub row: usize,

    pub input: Input,
}

impl RollInitiative {
    /// Returns the [`Input`] widget.
    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn help(&self) -> String {
        "<escape>: cancel, back to initiative tracker\n<enter>: set initiative\n<ctrl-enter>: set initiative and finish, sort combatants\n<up>: previous combatant\n<down>: next combatant".to_string()
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

    pub fn set_row(&mut self, row: usize, tracker: &mut Tracker) {
        self.row = row;
        tracker.highlight(row);
        self.name = tracker.combatant(row).map(|c| c.name().to_string());
    }

    pub fn init_tracker(&mut self, tracker: &mut Tracker) {
        self.set_row(0, tracker);
    }

    pub fn handle_event(&mut self, key: KeyEvent, tracker: &mut Tracker) -> Option<State> {
        let Some(unhandled_key) = self.input.update(key) else {
            return None;
        };
        match unhandled_key.code {
            KeyCode::Esc => Some(State::Home),
            KeyCode::Enter => {
                let initiative = self.input.take().parse().ok().unwrap_or_default();
                tracker.combatant_mut(self.row).map(|c| c.initiative = initiative);

                let next_row = (self.row + 1) % tracker.combatants().len();
                self.set_row(next_row, tracker);
                None
            },
            KeyCode::Down => {
                let next_row = (self.row + 1) % tracker.combatants().len();
                self.set_row(next_row, tracker);
                None
            },
            KeyCode::Up => {
                let prev_row = self.row.wrapping_sub(1) % tracker.combatants().len();
                self.set_row(prev_row, tracker);
                None
            },
            // enter seems to return ctrl-j
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let initiative = self.input.take().parse().ok().unwrap_or_default();
                tracker.combatant_mut(self.row).map(|c| c.initiative = initiative);

                tracker.sort();

                Some(State::Home)
            },
            _ => None,
        }
    }
}
