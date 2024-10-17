use crate::{input::Input, view::tracker::{combatant::Combatant, state::State, Tracker}};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};

/// Adding a new combatant to the initiative order.
#[derive(Debug, PartialEq, Eq)]
pub struct AddCombatant {
    /// The name of the combatant.
    pub name: Option<String>,

    /// The hit points of the combatant.
    pub hit_points: Option<i32>,

    /// The current row being edited.
    pub row: usize,

    input: Input,
}

impl Default for AddCombatant {
    fn default() -> Self {
        Self {
            name: None,
            hit_points: None,
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

impl AddCombatant {
    /// Returns the [`Input`] widget.
    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn help(&self) -> String {
        "<escape>: cancel, back to initiative tracker
<enter>: add combatant
<ctrl-enter>: add combatant and finish
+ or =: next field
- or _: previous field".to_string()
    }

    pub fn render(&self) -> Table {
        fn highlight<'a>(name: &'a str, value: &'a str) -> Row<'a> {
            Row::new([Text::from(name), Text::from(value)])
                .style(Style::default().bg(Color::Rgb(0, 48, 130)))
        }

        let maybe_highlight = |do_highlight, name, value| if do_highlight {
            highlight(name, self.input.as_str())
        } else {
            Row::new([Text::from(name), Text::from(value)])
        };
        Table::new(
            [
                maybe_highlight(self.row == 0, "Name", self.name.as_ref().map(|name| name.to_string()).unwrap_or_default()),
                maybe_highlight(self.row == 1, "Hit Points", self.hit_points.as_ref().map(|hp| hp.to_string()).unwrap_or_default()),
            ],
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .padding(Padding::horizontal(1))
                    .title("Add Combatant")
            )
    }

    pub fn set_row_idx(&mut self, row: usize) {
        self.row = row;
        match self.row {
            0 => self.input.set(self.name.take().unwrap_or_default()),
            1 => self.input.set(self.hit_points.take().map(|hp| hp.to_string()).unwrap_or_default()),
            _ => (),
        }
    }

    pub fn set_row_content(&mut self, content: String) {
        if content.is_empty() {
            return;
        }

        match self.row {
            0 => self.name = Some(content),
            1 => self.hit_points = Some(content.parse().unwrap_or_default()),
            _ => (),
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent, tracker: &mut Tracker) -> Option<State> {
        let Some(unhandled_key) = self.input.update(key) else {
            return None;
        };
        match unhandled_key.code {
            KeyCode::Esc => Some(State::Home),
            KeyCode::Enter => {
                let content = self.input.take();
                self.set_row_content(content);

                let hp = self.hit_points.take().unwrap_or_default();
                tracker.add_combatant(Combatant::new(
                    self.name.take().unwrap_or_default(),
                    hp,
                    hp,
                ));

                self.set_row_idx(0);
                None
            },
            KeyCode::Char('+') | KeyCode::Char('=') => {
                let content = self.input.take();
                self.set_row_content(content);
                self.set_row_idx((self.row + 1) % 2);
                None
            },
            KeyCode::Char('-') | KeyCode::Char('_') => {
                let content = self.input.take();
                self.set_row_content(content);
                self.set_row_idx((self.row + 1) % 2);
                None
            },
            // enter seems to return ctrl-j
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let content = self.input.take();
                self.set_row_content(content);

                let hp = self.hit_points.take().unwrap_or_default();
                tracker.add_combatant(Combatant::new(
                    self.name.take().unwrap_or_default(),
                    hp,
                    hp,
                ));
                Some(State::Home)
            },
            _ => None,
        }
    }
}
