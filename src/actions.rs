/// The different actions available for a combatant and whether they are available.
pub struct Actions {
    /// The actions available for the combatant.
    pub actions: Vec<Action>,
}

impl std::fmt::Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((last, elements)) = self.actions.split_last() {
            for action in elements {
                write!(f, "{}/", action)?;
            }
            write!(f, "{}", last)?;
        }

        Ok(())
    }
}

impl Default for Actions {
    fn default() -> Actions {
        Actions {
            // not all creatures have bonus actions
            actions: vec![Action::Move, Action::Action, Action::Reaction],
        }
    }
}

impl Actions {
    /// Returns a styled [`Line`] displaying the actions available for the combatant.
    ///
    /// [`Line`]: ratatui::text::Line
    pub fn line(&self) -> ratatui::text::Line {
        self.actions.iter()
            .map(|action| action.span())
            .collect::<Vec<_>>()
            .into()
    }
}

/// An action that a combatant can take.
pub enum Action {
    /// Move up to the combatant's speed.
    Move,

    /// Take a standard action (e.g. attack, cast a spell, hide, etc.).
    Action,

    /// Take a bonus action (e.g. attack with offhand, cast a bonus action spell, drink a potion,
    /// Rogue's Cunning Action, etc.).
    BonusAction,

    /// Take a reaction (e.g. opportunity attack, shield spell, readied action, etc.).
    Reaction,
}

impl Action {
    /// Returns a styled [`Span`] displaying the action.
    ///
    /// [`Span`]: ratatui::text::Span
    pub fn span(&self) -> ratatui::text::Span {
        use ratatui::prelude::*;
        match self {
            Action::Move => Span::styled("M", Style::default().fg(Color::Green)),
            Action::Action => Span::styled("A", Style::default().fg(Color::Yellow)),
            Action::BonusAction => Span::styled("BA", Style::default().fg(Color::Blue)),
            Action::Reaction => Span::styled("R", Style::default().fg(Color::Red)),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move => write!(f, "{}", "M"),
            Action::Action => write!(f, "{}", "A"),
            Action::BonusAction => write!(f, "{}", "BA"),
            Action::Reaction => write!(f, "{}", "R"),
            // Action::Move => write!(f, "{}", "M".green()),
            // Action::Action => write!(f, "{}", "A".fg_rgb::<182, 165, 23>()),
            // Action::BonusAction => write!(f, "{}", "BA".blue()),
            // Action::Reaction => write!(f, "{}", "R".red()),
        }
    }
}
