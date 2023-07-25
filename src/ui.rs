use std::fs;

use ratatui::{backend::Backend, Frame, layout::{Layout, Constraint, Direction, Alignment, Rect}, widgets::{Block, BorderType, Borders, Wrap, Paragraph, Clear}, style::Style};

use crate::{App, app::io::InputMode};

mod button;
use button::*;
mod panel;
use panel::*;

#[derive(Clone)]
pub enum Panels{
    Chat,
    Query,
    Status,
}

#[derive(Clone)]
pub struct Ui{
    pub active_panel: Panels,

}

impl Ui{
    pub fn new() -> Self{
        Self{
            active_panel: Panels::Chat,
        }
    }

    pub fn draw<B:Backend>(f: &mut Frame<B>, app:&mut App){
        let window = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Min(1),
                Constraint::Percentage(80), 
                Constraint::Percentage(20),
                Constraint::Min(1),
            ].as_ref())
            .split(f.size());

        let system_layout_h = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ].as_ref())
            .split(f.size());
        let system_layout_v = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ].as_ref())
            .split(system_layout_h[1]);

        let menu_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ].as_ref())
            .split(window[0]);

        let menu_block = Block::default()
            .borders(Borders::NONE);



        
        let mut chat_panel = panel::chat_panel::new();
        match app.input.mode{
            InputMode::Normal => chat_panel.panel.active(true),
            _ => chat_panel.panel.active(false),
        };
        chat_panel.draw(window[1], f, app);

        let mut query_panel = panel::query_panel::new();
        match app.input.mode{
            InputMode::Editing => query_panel.panel.active(true),
            _ => query_panel.panel.active(false),
        };

        query_panel.draw(window[2], f, app);
        
        let mut status_panel = panel::status_panel::new();
        match app.input.mode{
            InputMode::Normal => status_panel.panel.active(true),
            _ => status_panel.panel.active(false),
        };
        status_panel.draw(window[3], f, app);
        
        let mut system_panel = panel::system_panel::new();
        match app.input.mode{
            InputMode::System => system_panel.panel.active(true),
            _ => system_panel.panel.active(false),
        };
        system_panel.draw(system_layout_v[1], f, app);
        //button.clicked();
        //f.render_widget(menu_bar, menu_layout[0]);
        //f.render_widget(chat_widget, window[1]);
        //f.render_widget(query_widget, window[2]);
        
        //f.render_widget(clear_widget, system_layout_v[1]);
        //f.render_widget(system_widget, system_layout_v[1]);
        
        //f.render_widget(status_bar, window[3]);

    }
}