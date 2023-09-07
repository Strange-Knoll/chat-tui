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
pub struct Ui<'a>{
    pub active_panel: Panels,
    pub chat_panel: panel::chat_panel<'a>,
    pub query_panel: panel::query_panel<'a>,
    pub system_panel: panel::system_panel<'a>,

}

impl Ui<'_>{
    pub fn new() -> Self{
        Self{
            active_panel: Panels::Chat,
            chat_panel: panel::chat_panel::new(),
            query_panel: panel::query_panel::new(),
            system_panel: panel::system_panel::new(),
        }
    }

    pub fn draw<B:Backend>(f: &mut Frame<B>, app:&mut App){
        let window = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Min(1),
                Constraint::Percentage(70), 
                Constraint::Percentage(30),
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
                Constraint::Percentage(33)
            ].as_ref())
            .split(window[0]);

        let menu_block = Block::default()
            .borders(Borders::NONE);

        let mut menu_bar = panel::menu_bar::new();
        menu_bar.draw(window[0], f, app);

        
        //Chat Panel
        match app.input.mode{
            InputMode::Normal => app.ui.chat_panel.panel.active(true),
            _ => app.ui.chat_panel.panel.active(false),
        };

        //
        // need to find a way to not clone app
        // 
        let mut chat_btn = Button::new(window[1], app.clone());
        if chat_btn.clicked(){
            app.input.mode = InputMode::Normal;
        }
        
        app.ui.chat_panel.draw(window[1], f);
    
        //
        //
        //

        //Query Panel
        match app.input.mode{
            InputMode::Editing => app.ui.query_panel.panel.active(true),
            _ => app.ui.query_panel.panel.active(false),
        };
        let mut query_btn = Button::new(window[2], app.clone());
        if query_btn.clicked(){
            app.input.mode = InputMode::Editing;
        }

        app.ui.query_panel.draw(window[2], f, &app.clone());
        
        //Status Panel
        let mut status_panel = panel::status_panel::new();
        match app.input.mode{
            InputMode::Normal => status_panel.panel.active(true),
            _ => status_panel.panel.active(false),
        };
        status_panel.draw(window[3], f, app);


        
        //System Panel
        match app.input.mode{
            InputMode::System => app.ui.system_panel.panel.active(true),
            _ => app.ui.system_panel.panel.active(false),
        };

        app.ui.system_panel.draw(system_layout_v[1], f, &mut app.clone());



        //Key Panel
        let mut key_panel = panel::key_panel::new();
        match app.input.mode {
            InputMode::ApiKey => key_panel.panel.active(true),
            _ => key_panel.panel.active(false),
        };
        key_panel.draw(system_layout_v[1], f, app)


        //button.clicked();
        //f.render_widget(menu_bar, menu_layout[0]);
        //f.render_widget(chat_widget, window[1]);
        //f.render_widget(query_widget, window[2]);
        
        //f.render_widget(clear_widget, system_layout_v[1]);
        //f.render_widget(system_widget, system_layout_v[1]);
        
        //f.render_widget(status_bar, window[3]);

    }
}