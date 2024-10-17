pub mod encounter;
pub mod tracker;

use crossterm::event::{KeyCode, KeyEvent};
use encounter::EncounterBuilder;
use ratatui::widgets::Widget;
use tracker::Tracker;

/// Whether [`State::transition`] did switch to a new state.
pub enum TransitionResult {
    /// We are still on the original state.
    Old,

    /// We transitioned to another state.
    New,
}

/// A state associated with a view.
pub trait State: std::fmt::Debug {
    /// Returns true if the state is the quit state.
    fn is_quit(&self) -> bool;

    /// Returns the default key event that triggers the state.
    fn default_key_event(&self) -> KeyCode;

    /// Returns true if the state needs keyboard input.
    fn needs_keyboard(&self) -> bool;

    /// Returns the help text for the state.
    fn help(&self) -> String;

    /// Potentially transitions to a new state given given a key event.
    fn transition(&mut self, key: KeyCode) -> TransitionResult;

    /// Renders the state to two [`Widget`]s, one for the state, and one for the input.
    fn render(&self) -> Option<(impl Widget, Option<impl Widget>)>;
}

/// A state transition declaration.
#[derive(Debug)]
pub struct Transition<S: State> {
    /// The state to transition to.
    pub state: S,

    /// The key that triggers the transition.
    pub key: KeyCode,
}

/// Creates a new transition declaration using the state's default key.
impl<S: State> From<S> for Transition<S> {
    fn from(state: S) -> Self {
        Self {
            key: state.default_key_event(),
            state,
        }
    }
}

impl State for encounter::state::State {
    fn is_quit(&self) -> bool {
        matches!(self, &encounter::state::State::Quit)
    }

    fn default_key_event(&self) -> KeyCode {
        encounter::state::State::default_key_event(self)
    }

    fn needs_keyboard(&self) -> bool {
        encounter::state::State::needs_keyboard(self)
    }

    fn help(&self) -> String {
        encounter::state::State::help(self)
    }

    fn transition(&mut self, key: KeyCode) -> TransitionResult {
        if let Some(new) = encounter::state::State::transition(self, key) {
            *self = new.state;
            TransitionResult::New
        } else {
            TransitionResult::Old
        }
    }

    fn render(&self) -> Option<(impl Widget, Option<impl Widget>)> {
        encounter::state::State::render(self)
    }
}

impl State for tracker::state::State {
    fn is_quit(&self) -> bool {
        matches!(self, tracker::state::State::Quit)
    }

    fn default_key_event(&self) -> KeyCode {
        tracker::state::State::default_key_event(self)
    }

    fn needs_keyboard(&self) -> bool {
        tracker::state::State::needs_keyboard(self)
    }

    fn help(&self) -> String {
        tracker::state::State::help(self)
    }

    fn transition(&mut self, key: KeyCode) -> TransitionResult {
        if let Some(new) = tracker::state::State::transition(self, key) {
            *self = new.state;
            TransitionResult::New
        } else {
            TransitionResult::Old
        }
    }

    fn render(&self) -> Option<(impl Widget, Option<impl Widget>)> {
        tracker::state::State::render(self)
    }
}

/// An object that serves as the main view for the program.
pub trait View {
    /// The state type for the view.
    type State: State;

    /// Returns the default state for the view.
    fn default_state() -> Self::State;

    /// Returns true if the given state is the quit state.
    fn is_quit(state: &Self::State) -> bool {
        state.is_quit()
    }

    /// Receive an event from the keyboard and update the given state or view.
    fn handle_event(&mut self, event: KeyEvent, state: &mut Self::State) -> Option<Self::State>;

    /// Initalize the view with the given state.
    fn init(&mut self, state: &mut Self::State);

    /// Renders the view to a [`Widget`].
    fn render(&self) -> impl Widget;
}

impl<V: View> View for &mut V {
    type State = V::State;

    fn default_state() -> Self::State {
        V::default_state()
    }

    fn handle_event(&mut self, event: KeyEvent, state: &mut Self::State) -> Option<Self::State> {
        V::handle_event(self, event, state)
    }

    fn init(&mut self, state: &mut Self::State) {
        V::init(self, state);
    }

    fn render(&self) -> impl Widget {
        V::render(self)
    }
}

impl View for EncounterBuilder {
    type State = encounter::state::State;

    fn default_state() -> Self::State {
        encounter::state::State::Home
    }

    fn handle_event(&mut self, event: KeyEvent, state: &mut Self::State) -> Option<Self::State> {
        state.handle_event(event, self)
    }

    fn init(&mut self, state: &mut Self::State) {
        state.init_encounter(self);
    }

    fn render(&self) -> impl Widget {
        EncounterBuilder::render(self)
    }
}

impl View for Tracker {
    type State = tracker::state::State;

    fn default_state() -> Self::State {
        tracker::state::State::Home
    }

    fn handle_event(&mut self, event: KeyEvent, state: &mut Self::State) -> Option<Self::State> {
        state.handle_event(event, self)
    }

    fn init(&mut self, state: &mut Self::State) {
        state.init_tracker(self);
    }

    fn render(&self) -> impl Widget {
        Tracker::render(self)
    }
}
