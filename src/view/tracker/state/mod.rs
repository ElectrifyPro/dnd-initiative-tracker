mod combatant;
mod initiative;

use combatant::add::AddCombatant;
use crate::{any_widget::AnyWidget, view::Transition};
use crossterm::event::{KeyCode, KeyEvent};
use initiative::RollInitiative;
use super::Tracker;

fn fmt_key_code(key: KeyCode) -> String {
    match key {
        KeyCode::Char(c) => c.to_string(),
        _ => todo!(),
    }
}

/// Any state the initiative tracker can be in.
#[derive(Default, Debug, PartialEq, Eq)]
pub enum State {
    /// The home state, where the user can view the initiative order and launch any other state
    /// below.
    #[default]
    Home,

    /// Adding a new combatant to the initiative order.
    AddCombatant(AddCombatant),

    /// Roll initiative for all combatants.
    RollInitiative(RollInitiative),

    /// Special quit state, which exits the program.
    Quit,
}

impl State {
    // TODO: make a derive macro for this

    /// Declares the states that can be reached from this state.
    ///
    /// A transition declaration can also override the default key that triggers the transition.
    pub fn transitions(&self) -> Vec<Transition<State>> {
        match self {
            State::Home => vec![
                State::AddCombatant(AddCombatant::default()).into(),
                State::RollInitiative(RollInitiative::default()).into(),
                State::Quit.into(),
            ],
            State::AddCombatant(_) => vec![State::Home.into()],
            State::RollInitiative(_) => vec![State::Home.into()],
            State::Quit => vec![],
        }
    }

    /// Returns the state to transition to given a key event.
    pub fn transition(&self, key: KeyCode) -> Option<Transition<State>> {
        self.transitions()
            .into_iter()
            .find(|transition| transition.key == key)
    }

    /// Returns the default key that triggers this state from any other state.
    ///
    /// This can be overriden by other states to provide custom behavior.
    pub fn default_key_event(&self) -> KeyCode {
        match self {
            State::Home => KeyCode::Char('h'),
            State::AddCombatant(_) => KeyCode::Char('a'),
            State::RollInitiative(_) => KeyCode::Char('r'),
            State::Quit => KeyCode::Char('q'),
        }
    }

    /// Returns the description of the state.
    pub fn description(&self) -> &'static str {
        match self {
            State::Home => "back to initiative tracker",
            State::AddCombatant(_) => "add combatant to initiative order",
            State::RollInitiative(_) => "roll initiative!",
            State::Quit => "quit the program",
        }
    }

    /// Returns the default help message for the state.
    pub fn default_help(&self) -> String {
        self.transitions()
            .iter()
            .map(|transition| {
                format!(
                    "{}: {}",
                    fmt_key_code(transition.key),
                    transition.state.description()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Returns the help message for the state.
    pub fn help(&self) -> String {
        match self {
            State::AddCombatant(add) => add.help(),
            State::RollInitiative(roll) => roll.help(),
            _ => self.default_help(),
        }
    }

    /// Renders the state to two widgets, one for the state, and one for the input.
    pub fn render(&self) -> Option<(AnyWidget, Option<AnyWidget>)> {
        match self {
            State::AddCombatant(add) => Some((add.render().into(), Some(add.input().into()))),
            State::RollInitiative(roll) => Some((roll.render().into(), Some(roll.input().into()))),
            _ => None,
        }
    }

    /// Returns true if this state needs to handle keyboard events.
    pub fn needs_keyboard(&self) -> bool {
        match self {
            State::AddCombatant(_) => true,
            State::RollInitiative(_) => true,
            _ => false,
        }
    }

    /// Initialize the tracker when transitioning to this state, if necessary.
    pub fn init_tracker(&mut self, tracker: &mut Tracker) {
        match self {
            State::RollInitiative(roll) => roll.init_tracker(tracker),
            _ => (),
        }
    }

    /// Receive events from the keyboard.
    pub fn handle_event(&mut self, key: KeyEvent, tracker: &mut Tracker) -> Option<State> {
        match self {
            State::AddCombatant(add) => add.handle_event(key, tracker),
            State::RollInitiative(roll) => roll.handle_event(key, tracker),
            _ => None,
        }
    }
}
