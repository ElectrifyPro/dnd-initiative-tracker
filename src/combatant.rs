use crate::actions::Actions;
use ratatui::widgets::{Cell, Row};
use std::borrow::Cow;

/// A combatant in a combat encounter.
pub struct Combatant {
    /// The name of the combatant.
    name: String,

    /// The combatant's initiative roll.
    initiative: i32,

    /// The hit points of the combatant.
    hit_points: i32,

    /// The maximum hit points of the combatant.
    max_hit_points: i32,

    /// Temporary hit points that the combatant has.
    temp_hit_points: i32,

    /// The actions available for the combatant.
    actions: Actions,
}

impl Combatant {
    /// Creates a new combatant with the given name and hit points.
    pub fn new(name: String, hit_points: i32, max_hit_points: i32) -> Combatant {
        Combatant {
            name,
            initiative: 0,
            hit_points,
            max_hit_points,
            temp_hit_points: 0,
            actions: Actions::default(),
        }
    }

    /// Returns the combatant's initiative.
    pub fn initiative(&self) -> i32 {
        self.initiative
    }

    /// Builds the combatant's table row.
    pub fn row(&self) -> Row {
        Row::new([
            Cell::from(self.initiative.to_string()),
            Cell::from(&*self.name),
            Cell::from(self.actions.line()),
            Cell::from(format!("{} / {}", self.hit_points, self.max_hit_points)),
            Cell::from(self.temp_hit_points.to_string()),
        ])
    }
}
