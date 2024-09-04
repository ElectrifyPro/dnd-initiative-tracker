use ratatui::{prelude::*, widgets::*};

/// Adding a new combatant to the initiative order.
#[derive(Default, PartialEq, Eq)]
pub struct AddCombatant {
    /// The name of the combatant.
    pub name: Option<String>,

    /// The hit points of the combatant.
    pub hit_points: i32,
}

impl AddCombatant {
    pub fn render(&self) -> Table {
        Table::new(
            vec![
                Row::new([Text::from("Name"), Text::from(self.name.as_deref().unwrap_or(""))]),
                Row::new([Text::from("Hit Points"), Text::from(self.hit_points.to_string())]),
            ],
            vec![Constraint::Percentage(50), Constraint::Percentage(50)],
        )
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .padding(Padding::horizontal(1))
                    .title("Add Combatant")
            )
            .widths(&[Constraint::Length(20), Constraint::Length(20)])
    }
}
