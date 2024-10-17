use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use std::collections::HashSet;

/// Helper to provide Bash-like text input functionality.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Input {
    /// The current input buffer.
    buffer: String,

    /// The current cursor position.
    cursor: usize,

    /// Keys to ignore.
    ignore: HashSet<KeyCode>,
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // draw bordered box
        Widget::render(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White))
                .title("<enter> to submit"),
            area,
            buf,
        );
        let padding = 2;
        let cursor = self.cursor;
        let buffer = &self.buffer;

        // show prompt
        let x = area.x + padding;
        let y = area.y + padding / 2;
        buf.set_string(x, y, ">", Style::default());

        // show buffer, set fake cursor
        buf.set_string(x + padding, y, buffer, Style::default());
        buf.set_string(
            x + padding + cursor as u16,
            y,
            buffer.get(cursor..cursor + 1).unwrap_or(" "),
            Style::default().add_modifier(Modifier::REVERSED),
        );
    }
}

impl Input {
    /// Set the keys to ignore.
    pub fn with_ignore(mut self, ignore: HashSet<KeyCode>) -> Self {
        self.ignore = ignore;
        self
    }

    /// Updates the input given a key event.
    ///
    /// Returns the key event if it was not consumed by the input.
    pub fn update(&mut self, event: KeyEvent) -> Option<KeyEvent> {
        if self.ignore.contains(&event.code) {
            return Some(event);
        }

        match event.code {
            KeyCode::Char(c) if !event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.buffer.insert(self.cursor, c);
                self.cursor += 1;
            },
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.buffer.remove(self.cursor);
                }
            },
            KeyCode::Delete => { 
                if self.cursor < self.buffer.len() {
                    self.buffer.remove(self.cursor);
                }
            },
            KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
            KeyCode::Right => self.cursor = self.cursor.saturating_add(1).min(self.buffer.len()),
            _ => return Some(event),
        }

        None
    }

    /// Takes the input from the buffer and clears it.
    pub fn take(&mut self) -> String {
        self.cursor = 0;
        std::mem::take(&mut self.buffer)
    }

    /// Returns a string slice of the input buffer.
    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Set the input buffer, moving the cursor to the end.
    pub fn set(&mut self, buffer: String) {
        self.buffer = buffer;
        self.cursor = self.buffer.len();
    }
}
