mod combatant;

use combatant::add::AddCombatant;

/// Any state the initiative tracker can be in.
#[derive(Default)]
pub enum State {
    /// The home state, where the user can view the initiative order and launch any other state
    /// below.
    #[default]
    Home,

    /// Adding a new combatant to the initiative order.
    AddCombatant(AddCombatant),
}

impl State {

}
