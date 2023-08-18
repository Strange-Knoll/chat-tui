
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use ratatui::Frame;
use ratatui::backend::Backend;
use ratatui::buffer::Buffer;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::layout::Rect;

use crate::app::App;

pub struct Button<'a> {
    pub rect: Rect,
    pub app: App<'a>,
}

impl Button <'_> {
    pub fn new(rect:Rect, app:App<'_>) -> Button<'_> {
        Button {
            rect,
            app,
        }
    }


    pub fn mouse_in_rect(&self, x:u16, y:u16) -> bool {
        if x >= self.rect.x && x <= self.rect.x + self.rect.width {
            if y >= self.rect.y && y <= self.rect.y + self.rect.height {
                return true;
            }
        }
        false
    }

    pub fn clicked (&mut self) -> bool {
        let mut out = false;
        if self.app.input.mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if self.mouse_in_rect(self.app.input.mouse.column, self.app.input.mouse.row) {
                out = true;
            }
        }
        out
    }

}
