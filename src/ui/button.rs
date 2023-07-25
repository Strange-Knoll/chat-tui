
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use ratatui::Frame;
use ratatui::backend::Backend;
use ratatui::buffer::Buffer;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::layout::Rect;

pub struct Button {
    pub rect: Rect,
}

impl Button {
    pub fn new(rect:Rect) -> Self {
        Self {
            rect: rect,
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

    pub fn clicked (&self) -> bool {
        let mut out = false;

        match event::read().unwrap() {
            Event::Mouse(event) => {
                if event.kind == MouseEventKind::Down(MouseButton::Left) {
                    if self.mouse_in_rect(event.column, event.row) {
                        out = true;
                    }
                }
            },
            _ => {}
        }
        out
    }

}
