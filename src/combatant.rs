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
}

impl Combatant {
    /// Builds the combatant's table record.
    pub fn record(&self) -> [String; 2] {
        [
            self.initiative.to_string(),
            self.name.clone(),
            // format!("{}/{}", self.hit_points, self.max_hit_points),
            // self.temp_hit_points.to_string(),
        ]
    }
}
