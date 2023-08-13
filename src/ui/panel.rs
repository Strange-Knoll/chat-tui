use std::fs;

use ratatui::{widgets::{Block, Paragraph, Borders, BorderType, Clear, Wrap}, layout::{Rect, Alignment}, Frame, backend::Backend, style::{Color, Style, Modifier}};

use crate::{App, app::{io::InputMode, self}};

#[derive(Clone)]
pub struct Panel<'a>{
    active: bool,
    active_block: Option<Block<'a>>,
    inactive_block: Option<Block<'a>>
}


impl<'a> Panel<'a>{
    pub fn new() -> Self{
        Self{
            active: false,
            active_block: None,
            inactive_block: None,
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

    pub fn draw<B:Backend>(&self, rect:Rect, f: &mut Frame<B>, app: &App){
        let log_string = fs::read_to_string("logs/chat.txt").unwrap();

        let mut para = Paragraph::new(log_string.as_str())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        if self.panel.active {
            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone())
                    .scroll(app.input.cursor_pos);
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

    pub fn draw<B:Backend>(&self, rect:Rect, f: &mut Frame<B>, app: &App){
        //let log_string = fs::read_to_string("logs/chat.txt").unwrap();
        let mut para = Paragraph::new(format!("{}_",app.input.query.as_str()))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        if self.panel.active {
            match self.panel.active_block {
                Some(ref block) => {
                    para = para.block(block.clone())
                    .scroll(app.input.cursor_pos);
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

    pub fn draw<B:Backend>(&self, rect:Rect, f: &mut Frame<B>, app: &App){
        //let log_string = fs::read_to_string("logs/system.txt").unwrap();       
        
        let mut para = Paragraph::new(format!("{}_",app.input.system.as_str()))
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