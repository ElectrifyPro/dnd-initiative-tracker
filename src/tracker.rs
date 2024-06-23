use super::State;

/// Manages the initiative tracker.
pub struct Tracker {
    /// The current state of the tracker, indicating what actions the user can take at the current
    /// moment.
    state: State,
}

impl Tracker {
    /// Creates a new initiative tracker.
    pub fn new() -> Tracker {
        Tracker {
            state: State::Home,
        }
    }
}
