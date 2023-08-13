use std::fs;

use crossterm::event::{MouseEventKind, MouseButton};
use ratatui::{widgets::{Block, Paragraph, Borders, BorderType, Clear, Wrap}, layout::{Rect, Alignment, Constraint, Layout, Direction}, Frame, backend::Backend, style::{Color, Style, Modifier}};

use crate::{App, app::{io::InputMode, self}};

use super::button::Button;

#[derive(Clone)]
pub struct Panel<'a>{
    pub active: bool,
    pub active_block: Option<Block<'a>>,
    pub inactive_block: Option<Block<'a>>,
    pub scroll: (u16, u16),
}


impl<'a> Panel<'a>{
    pub fn new() -> Self{
        Self{
            active: false,
            active_block: None,
            inactive_block: None,
            scroll: (0,0),
        }
    }

    pub fn active(&mut self, active:bool) -> &mut Self{
        self.active = active;
        self
    }

    pub fn active_block(&mut self, block:Block<'a>) -> Panel<'a>{
        self.active_block = Some(block);
        self.clone()
    }

    pub fn inactive_block(&mut self, block:Block<'a>) -> Panel<'a>{
        self.inactive_block = Some(block);
        self.clone()
    }
}

pub struct menu_bar<'a>{
    pub panel: Panel<'a>
}

impl menu_bar<'_>{
    pub fn new() -> Self{
        Self{
            panel:Panel::new()
                .active(false)
                .active_block(
                    Block::default()
                        .borders(Borders::NONE)
                        .style(Style::default().fg(Color::Magenta))
                ).clone()
                .inactive_block(
                    Block::default()
                        .borders(Borders::NONE)
                        .style(Style::default().fg(Color::White))
                ).clone()
        }
    }
    pub fn draw<B:Backend>(&self, rect:Rect, f:&mut Frame<B>, app:&mut App){
        let mut key = Paragraph::new("Api Key");
        let mut system = Paragraph::new("System");
        let mut assistant = Paragraph::new("Assistant");
        let mut help = Paragraph::new("Help");

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints( vec![
                Constraint::Length(" Api Key ".len() as u16),
                Constraint::Length(" System ".len()as u16),
                Constraint::Length(" Assistant ".len() as u16),
                Constraint::Length(" Help ".len()as u16),
            ].as_ref())
            .split(rect);
        
        let mut key_btn = Button::new(layout[0], app.clone());
        let mut system_btn = Button::new(layout[1], app.clone());
        let mut assistant_btn = Button::new(layout[2], app.clone());
        let mut help_btn = Button::new(layout[3], app.clone());
        
        if key_btn.clicked(){
            key = key.clone().block(self.panel.active_block.as_ref().unwrap().clone());
            app.input.mode = InputMode::ApiKey;
        }else {
            key = key.clone().block(self.panel.inactive_block.as_ref().unwrap().clone());
        }

        if system_btn.clicked(){
            system = system.clone().block(self.panel.active_block.as_ref().unwrap().clone());
            app.input.mode = InputMode::System;
        }else {
            system = system.clone().block(self.panel.inactive_block.as_ref().unwrap().clone());
        }

        /*if assistant_btn.clicked(){
            assistant = assistant.clone().block(self.panel.active_block.as_ref().unwrap().clone());
        }else {
            assistant = assistant.clone().block(self.panel.inactive_block.as_ref().unwrap().clone());
        }
        if help_btn.clicked(){
            help = help.clone().block(self.panel.active_block.as_ref().unwrap().clone());
        }else {
            help = help.clone().block(self.panel.inactive_block.as_ref().unwrap().clone());
        }*/

        
        f.render_widget(key, layout[0]);
        f.render_widget(system, layout[1]);
        //f.render_widget(assistant, layout[2]);
        //f.render_widget(help, layout[3]);
                
    }
}

#[derive(Clone)]
pub struct chat_panel<'a>{
    pub panel: Panel<'a>,
}

impl chat_panel<'_>{
    pub fn new() -> Self{
        Self{
            panel: Panel::new()
                .active(true)
                .active_block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Magenta))
                    .title("Chat")
                ).clone()
                .inactive_block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .title("Chat")
                ).clone(),
        }
    }

    pub fn draw<B:Backend>(&mut self, rect:Rect, f: &mut Frame<B>){
        let log_string = fs::read_to_string("logs/chat.txt").unwrap();

        let mut para = Paragraph::new(log_string.as_str())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((self.panel.scroll.0, self.panel.scroll.1));

        
        if self.panel.active {
            /*match app.input.mouse.kind {
                MouseEventKind::ScrollUp => {
                    self.panel.scroll.0 += 1;
                }
                MouseEventKind::ScrollDown => {
                    if self.panel.scroll.0 > 0{
                        self.panel.scroll.0 -= 1;
                    }
                }
                _ => {}
                
            }*/

            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                    //.scroll((0, app.input.cursor_pos.1));
                },
                None => {},
            }
        } else{
            match self.panel.inactive_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                },
                None => {},
            }
        }
/*         let mut button = Button::new(rect, app.clone());
        if button.clicked(){
            app.input.mode(InputMode::Normal);
        } */

        f.render_widget(para, rect);
    }
}


#[derive(Clone)]
pub struct query_panel<'a>{
    pub panel: Panel<'a>,
}

impl <'a> query_panel <'a>{
    pub fn new() -> Self{
        Self{
            panel: Panel::new()
                .active(false)
                .active_block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Magenta))
                    .title("Ask Question"))
                .inactive_block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::White))
                    .title("Ask Question")
                ).clone(),
        }
    }

    pub fn draw<B:Backend>(&mut self, rect:Rect, f: &mut Frame<B>, app:&App){
        //let log_string = fs::read_to_string("logs/chat.txt").unwrap();
        let mut para = Paragraph::new(format!("{}_",app.input.query.as_str()))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((self.panel.scroll.0, self.panel.scroll.1));

        
        if self.panel.active {
            /*match app.input.mouse.kind {
                MouseEventKind::ScrollUp => {
                    self.panel.scroll.0 = 1;
                }
                MouseEventKind::ScrollDown => {
                    if self.panel.scroll.0 > 0{
                        self.panel.scroll.0 =0;
                    }
                }
                _ => {}
                
            }*/
            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone())
                    
                },
                None => {},
            }
        } else{
            match self.panel.inactive_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                },
                None => {},
            }
        }
        /* let mut button = Button::new(rect, app.clone());
        if button.clicked(){
            app.input.mode(InputMode::Normal);
        }
 */
        f.render_widget(para, rect);
    }
}

#[derive(Clone)]
pub struct status_panel<'a>{
    pub panel: Panel<'a>,
}

impl <'a> status_panel <'a>{
    pub fn new() -> Self{
        Self{
            panel: Panel::new()
                .active(false)
                .clone()
        }
    }

    pub fn draw<B:Backend>(&self, rect:Rect, f: &mut Frame<B>, app: &App){
        //let log_string = fs::read_to_string("logs/chat.txt").unwrap();
        let mut para = Paragraph::new(format!("Mode: {}", 
            match app.input.mode{
                InputMode::Normal => "Normal: (a) ask question (k) set api key (s) open system panel (enter) send query (q) exit",
                InputMode::Editing => "Editing: (esc) return to Normal mode",
                InputMode::System => "System: (esc) return to Normal mode",
                InputMode::Assistant => "Assistant: (esc) return to Normal mode",
                InputMode::ApiKey => "Api Key: (enter) paste from clipboard (esc) return to Normal"
            }))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .style(Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD));

        if self.panel.active {
            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                },
                None => {},
            }
        } else{
            match self.panel.inactive_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                },
                None => {},
            }
        }

        f.render_widget(para, rect);
    }
}

#[derive(Clone)]
pub struct system_panel<'a>{
    pub panel: Panel<'a>,
}

impl <'a> system_panel <'a>{
    pub fn new() -> Self{
        Self{
            panel: Panel::new()
                .active(false)
                .active_block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green))
                    .title("System")
                ).clone()
                .inactive_block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("System")
                ).clone(),
        }
    }

    pub fn draw<B:Backend>(&mut self, rect:Rect, f: &mut Frame<B>, app: &App){
        //let log_string = fs::read_to_string("logs/system.txt").unwrap();       
        
        let mut para = Paragraph::new(format!("{}_",app.input.system.as_str()))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((self.panel.scroll.0, self.panel.scroll.1));

        

        if self.panel.active {
            /*match app.input.mouse.kind {
                MouseEventKind::ScrollUp => {
                    self.panel.scroll.0 =1;
                }
                MouseEventKind::ScrollDown => {
                    if self.panel.scroll.0 > 0{
                        self.panel.scroll.0 = 0;
                    }
                }
                _ => {}
                
            }*/
            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                    f.render_widget(Clear, rect);
                    f.render_widget(para, rect);
                },
                None => {},
            }
        } else{
            match self.panel.inactive_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                },
                None => {},
            }
        }
       
    }
}

pub struct key_panel<'a>{
    pub panel:Panel<'a>
}

impl <'a> key_panel <'a>{
    pub fn new() -> Self{
        Self { 
            panel: Panel::new()
                .active(false)
                .active_block(
                    Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(Color::Green))
                    .title("Api Key")
                )
                .inactive_block(
                    Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(Color::White))
                    .title("Api Key")
                )
        }
    }

    pub fn draw<B:Backend>(&self, rect:Rect, f:&mut Frame<B>, app: &App){
        let mut para = Paragraph::new(format!("{}", app.input.key.as_str()))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        if self.panel.active {
            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                    f.render_widget(Clear, rect);
                    f.render_widget(para, rect);
                },
                None => {},
            }
        } else{
            match self.panel.inactive_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                },
                None => {},
            }
        }
    }
}

pub struct key_panel<'a>{
    pub panel:Panel<'a>
}

impl <'a> key_panel <'a>{
    pub fn new() -> Self{
        Self { 
            panel: Panel::new()
                .active(false)
                .active_block(
                    Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(Color::Green))
                    .title("Api Key")
                )
                .inactive_block(
                    Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(Color::White))
                    .title("Api Key")
                )
        }
    }

    pub fn draw<B:Backend>(&self, rect:Rect, f:&mut Frame<B>, app: &App){
        let mut para = Paragraph::new(format!("{}", app.input.key.as_str()))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        if self.panel.active {
            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                    f.render_widget(Clear, rect);
                    f.render_widget(para, rect);
                },
                None => {},
            }
        } else{
            match self.panel.inactive_block {
                Some(ref block) => {
                    para = para.block(block.clone());
                },
                None => {},
            }
        }
    }
}