mod combatant;

use combatant::add::AddCombatant;
use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};

fn fmt_key_code(key: KeyCode) -> String {
    match key {
        KeyCode::Char(c) => c.to_string(),
        _ => todo!(),
    }
}

/// A state transition declaration.
pub struct Transition {
    /// The state to transition to.
    pub state: State,

    /// The key that triggers the transition.
    pub key: KeyCode,
}

/// Creates a new transition declaration using the state's default key.
impl From<State> for Transition {
    fn from(state: State) -> Self {
        Self {
            key: state.default_key_event(),
            state,
        }
    }
}

/// A widget that can be rendered to the terminal.
pub enum AnyWidget<'a> {
    Table(Table<'a>),
}

impl Widget for AnyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized
    {
        match self {
            AnyWidget::Table(table) => Widget::render(table, area, buf),
        }
    }
}

impl<'a> From<Table<'a>> for AnyWidget<'a> {
    fn from(table: Table<'a>) -> Self {
        AnyWidget::Table(table)
    }
}

/// Any state the initiative tracker can be in.
#[derive(Default, PartialEq, Eq)]
pub enum State {
    /// The home state, where the user can view the initiative order and launch any other state
    /// below.
    #[default]
    Home,

    /// Adding a new combatant to the initiative order.
    AddCombatant(AddCombatant),

    /// Special quit state, which exits the program.
    Quit,
}

impl State {
    // TODO: make a derive macro for this

    /// Declares the states that can be reached from this state.
    ///
    /// A transition declaration can also override the default key that triggers the transition.
    pub fn transitions(&self) -> Vec<Transition> {
        match self {
            State::Home => vec![State::AddCombatant(AddCombatant::default()).into(), State::Quit.into()],
            State::AddCombatant(_) => vec![State::Home.into()],
            State::Quit => vec![],
        }
    }

    /// Returns the state to transition to given a key event.
    pub fn transition(&self, key: KeyCode) -> Option<Transition> {
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
            State::Quit => KeyCode::Char('q'),
        }
    }

    /// Returns the description of the state.
    pub fn description(&self) -> &'static str {
        match self {
            State::Home => "back to initiative tracker",
            State::AddCombatant(_) => "add combatant to initiative order",
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

    /// Renders the state to a widget.
    pub fn render(&self) -> Option<AnyWidget> {
        match self {
            State::AddCombatant(add) => Some(add.render().into()),
            _ => None,
        }
    }
}
