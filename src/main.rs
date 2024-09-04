//! A command line initiative tracker for Dungeons and Dragons 5th Edition.
//!
//! This tracker allows you, the Dungeon Master, to keep track of the initiative order of the
//! players and monsters in a combat encounter. You can add and remove combatants during the
//! encounter, add and remove conditions from combatants, track spells and abilities, and track the
//! hit points of each combatant.
//!
//! The tracker uses simple Vim-like commands to navigate the interface and perform actions. If
//! you ever get stuck in a weird state, you can always type `h` to see a list of available
//! commands for the current context or `u` to undo the last command.

mod combatant;
mod state;
mod tracker;
mod ui;

use combatant::Combatant;
use crossterm::event::{read, Event};
use state::State;
use ui::Ui;
use tracker::Tracker;

fn main() -> std::io::Result<()> {
    let mut ui = Ui::new()?;
    let mut tracker = Tracker::new();
    let mut state = State::Home;

    loop {
        if state == State::Quit {
            break;
        }

        ui.render(&tracker, &state)?;
        match read()? {
            Event::Key(event) => {
                if let Some(transition) = state.transition(event.code) {
                    state = transition.state;
                }
            },
            _ => (),
        }
    }

    Ok(())
}
