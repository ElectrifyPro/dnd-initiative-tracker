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

mod actions;
mod any_widget;
mod input;
mod ui;
mod view;

use crossterm::event::{read, Event};
use ui::Ui;
use view::{encounter::EncounterBuilder, tracker::Tracker, State, TransitionResult, View};

fn render_view<S, V>(ui: &mut Ui, state: &mut S, view: &mut V) -> std::io::Result<()>
where
    S: State,
    V: View<State = S>,
{
    loop {
        if V::is_quit(&state) {
            break;
        }

        ui.render_view(view, state)?;
        let Event::Key(event) = read()? else {
            continue;
        };
        if state.needs_keyboard() {
            if let Some(new_state) = view.handle_event(event, state) {
                *state = new_state;
                view.init(state);
            }
        } else {
            if let TransitionResult::New = state.transition(event.code) {
                view.init(state);
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut ui = Ui::new()?;
    let mut encounter = EncounterBuilder::new();
    let mut state = EncounterBuilder::default_state();

    render_view(&mut ui, &mut state, &mut encounter)?;

    // let mut ui = Ui::new()?;
    // let mut tracker = Tracker::new();
    // let mut state = State::Home;
    //
    // render_view(&mut ui, &mut state, &mut tracker)?;

    Ok(())
}
