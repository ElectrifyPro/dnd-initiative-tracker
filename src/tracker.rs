use super::{Combatant, State};
use tabled::{
    builder::Builder,
    settings::{Modify, object::Rows, Alignment, Style}
};

/// Manages the initiative tracker.
#[derive(Default)]
pub struct Tracker {
    /// The current state / view of the tracker, indicating what actions the user can take at the
    /// current moment.
    state: State,

    /// The combatants of the encounter, ordered by initiative. The first combatant in the list is
    /// the combatant with the highest initiative.
    combatants: Vec<Combatant>,
}

impl std::fmt::Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // print the combatants in a table
        let mut builder = Builder::default();

        // table header:
        // 0: combatant's initiative
        // 1: combatant's name
        builder.push_record([
            "Initiative",
            "Name",
        ]);

        if self.combatants.is_empty() {
            builder.push_record(["empty"; 1]);
        } else {
            self.combatants.iter().for_each(|combatant| {
                builder.push_record(combatant.record());
            });
        }

        let mut table = builder.build();
        table.with(Style::rounded())
            .modify(Rows::new(1..), Alignment::left());
        write!(f, "{}", table)
    }
}

impl Tracker {
    /// Creates a new initiative tracker.
    pub fn new() -> Tracker {
        Tracker::default()
    }
}
