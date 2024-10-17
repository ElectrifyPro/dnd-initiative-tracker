use ratatui::widgets::{Cell, Row};

/// A combatant being built for the combat encounter.
///
/// All fields are optional, and will be filled in with default values at the start of the combat
/// encounter if they are not provided.
#[derive(Default)]
pub struct Combatant {
    /// The name of the combatant.
    pub name: Option<String>,

    /// The combatant's initiative roll.
    pub initiative: Option<i32>,

    /// The hit points of the combatant.
    pub hit_points: Option<i32>,

    /// The maximum hit points of the combatant.
    pub max_hit_points: Option<i32>,

    /// Temporary hit points that the combatant has.
    pub temp_hit_points: Option<i32>,
}

impl Combatant {
    /// Possibly sets the name of the combatant.
    pub fn name(mut self, name: Option<impl Into<String>>) -> Self {
        self.name = name.map(Into::into);
        self
    }

    /// Possibly sets the initiative of the combatant.
    pub fn initiative(mut self, initiative: Option<i32>) -> Self {
        self.initiative = initiative;
        self
    }

    /// Possibly sets the hit points of the combatant.
    pub fn hit_points(mut self, hit_points: Option<i32>) -> Self {
        self.hit_points = hit_points;
        self
    }

    /// Possibly sets the maximum hit points of the combatant.
    pub fn max_hit_points(mut self, max_hit_points: Option<i32>) -> Self {
        self.max_hit_points = max_hit_points;
        self
    }

    /// Possibly sets the temporary hit points of the combatant.
    pub fn temp_hit_points(mut self, temp_hit_points: Option<i32>) -> Self {
        self.temp_hit_points = temp_hit_points;
        self
    }

    /// Builds the combatant's table row.
    pub fn row(&self) -> Row {
        Row::new([
            Cell::from(self.name.as_deref().unwrap_or_default()),
            Cell::from(format!(
                "{} / {}",
                self.hit_points.map(|hp| hp.to_string()).unwrap_or_else(|| "?".to_string()),
                self.max_hit_points.map(|mhp| mhp.to_string()).unwrap_or_else(|| "?".to_string())
            )),
            Cell::from(self.temp_hit_points.map(|thp| thp.to_string()).unwrap_or_default()),
        ])
    }
}
