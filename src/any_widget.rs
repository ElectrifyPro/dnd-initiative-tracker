use crate::input::Input;
use ratatui::{prelude::*, widgets::*};

/// A widget that can be rendered to the terminal.
pub enum AnyWidget<'a> {
    Table(Table<'a>),
    Input(&'a Input),
}

impl Widget for AnyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized
    {
        match self {
            AnyWidget::Table(table) => Widget::render(table, area, buf),
            AnyWidget::Input(input) => Widget::render(input, area, buf),
        }
    }
}

impl<'a> From<Table<'a>> for AnyWidget<'a> {
    fn from(table: Table<'a>) -> Self {
        AnyWidget::Table(table)
    }
}

impl<'a> From<&'a Input> for AnyWidget<'a> {
    fn from(input: &'a Input) -> Self {
        AnyWidget::Input(input)
    }
}
