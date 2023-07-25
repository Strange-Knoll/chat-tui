use std::{error::Error, fs::{File, self}, io::Write};

use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseButton, MouseEventKind};

use crate::{App, ai};
#[derive(Clone)]
pub struct Input{
    pub mode: InputMode,
    pub query: String,
    pub system: String,
    pub cursor_pos:(u16, u16),
}
#[derive(Clone)]
pub struct Output{
    pub logs_path: String,
}

#[derive(Clone)]
pub enum InputMode{
    Normal,
    Editing,
    System 
}

impl Input{
    pub fn new() -> Self{
        Self{
            mode: InputMode::Normal,
            query: String::new(),
            system: fs::read_to_string("logs/system.txt").unwrap(),
            cursor_pos: (0,0),
        }
    }


    pub async fn key(&mut self, app: &mut App, key:KeyEvent) ->  Result<&mut Self, Box<dyn Error>>{
        match self.mode{
            InputMode::Normal => {
                match key.code{
                    KeyCode::Char('a') => {
                        self.mode = InputMode::Editing;
                    },
                    KeyCode::Char('s') => {
                        self.mode = InputMode::System;
                    },
                    KeyCode::Enter => {
                        app.ai = app.ai.clone().system( self.system.clone()).clone();
                        let response = app.ai.request(self.query.clone()).await?;
                        self.query = String::new();
                        Output::write_file(app.output.logs_path.clone(), response);
                    }, 
                    /*KeyCode::Up => {
                        if self.cursor_pos.0>0{
                            self.cursor_pos.0 -= 1;
                        }
                    },
                    KeyCode::Down => {
                        self.cursor_pos.0 += 1;
                    },
                    KeyCode::Left => {
                        if self.cursor_pos.1>0{
                            self.cursor_pos.1 -= 1;
                        }
                    },
                    KeyCode::Right => {
                        self.cursor_pos.1 += 1;
                    },*/
                    _ => {}
                }
            },
            InputMode::Editing => {
                match key.code{
                    KeyCode::Esc => {
                        self.mode = InputMode::Normal;
                    },
                    KeyCode::Enter => {
                        self.query.push('\n');
                    },
                    KeyCode::Char(c) => {
                        self.query.push(c);
                    },
                    KeyCode::Backspace => {
                        self.query.pop();
                    },
                    _ => {}
                }
            },
            InputMode::System => {
                match key.code{
                    KeyCode::Esc => {
                        Output::write_file("logs/system.txt".to_owned(), self.system.clone());
                        self.mode = InputMode::Normal;
                    },
                    KeyCode::Enter => {
                        self.system.push('\n');
                    },
                    KeyCode::Char(c) => {
                        self.system.push(c);
                    },
                    KeyCode::Backspace => {
                        self.system.pop();
                    },
                    _ => {}
                }
            }

        }
        
        Ok(self)
    }

    pub async fn mouse(&mut self, app:&mut App, event:MouseEvent) -> Result<&mut Self, Box<dyn Error>>{
        match event.kind{
            MouseEventKind::Down(MouseButton::Left) => {
                //self.mode = InputMode::Editing;
            },
            MouseEventKind::ScrollUp => {
                if self.cursor_pos.0>0{
                    self.cursor_pos.0 -= 2;
                }
            },
            MouseEventKind::ScrollDown => {
                self.cursor_pos.0 += 2;
            },
            _ => {}
        }
        
        Ok(self)
    }
}

impl Output{
    pub fn new() -> Self{
        Self{
            logs_path: String::new(),
        }
    }

    pub fn logs_path(&mut self, logs_path: String) -> Self{
        self.logs_path = logs_path;
        self.clone()
    }

    pub fn write_file(path:String, content:String){
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    } 
}