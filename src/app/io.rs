use std::{error::Error, fs::{File, self}, io::Write};

use async_openai::Client;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseButton, MouseEventKind, Event, self};

use crate::{App, ai};
#[derive(Clone)]
pub struct Input{
    pub mode: InputMode,
    pub query: String,
    pub system: String,
    pub assistant: String,
    pub key:String,
    pub cursor_pos:(u16, u16),
    pub mouse: MouseEvent,
}
#[derive(Clone)]
pub struct Output{
    pub logs_path: String,
}

#[derive(Clone)]
pub enum InputMode{
    Normal,
    Editing,
    System,
    Assistant,
    ApiKey
}

impl Input{
    pub fn new() -> Self{
        Self{
            mode: InputMode::Normal,
            query: String::new(),
            system: fs::read_to_string("logs/system.txt").unwrap(),
            assistant: fs::read_to_string("logs/assistant.txt").unwrap(),
            key: fs::read_to_string("logs/key.txt").unwrap(),
            cursor_pos: (0,0),
            mouse: MouseEvent{kind:MouseEventKind::Moved, column:0, row:0, modifiers:crossterm::event::KeyModifiers::NONE},
        }
    }

    pub fn mode(&mut self, mode: InputMode) -> Self{
        self.mode = mode;
        self.clone()
    }


    pub async fn key(&mut self, app: &mut App<'_>, key:KeyEvent) ->  Result<&mut Self, Box<dyn Error>>{
        match self.mode{
            InputMode::Normal => {
                match key.code{
                    KeyCode::Char('a') => {
                        self.mode = InputMode::Editing;
                    },
                    KeyCode::Char('s') => {
                        self.mode = InputMode::System;
                    },
                    KeyCode::Char('k') => {
                        self.mode = InputMode::ApiKey;
                    }
                    KeyCode::Enter => {
                        app.ai = app.ai.clone()
                            .system( self.system.clone())
                            .client(Client::new().with_api_key(self.key.clone()))
                            .clone();
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

            InputMode::ApiKey => {
                match key.code{
                    KeyCode::Esc => {
                        self.mode = InputMode::Normal;
                        Output::write_file("logs/key.txt".to_owned(), self.key.clone());
                        app.ai = app.ai.clone().client(
                            Client::new().with_api_key(self.key.clone())
                        ).clone();
                    }
                    KeyCode::Char(c) => {
                        self.key.push(c);
                    },
                    KeyCode::Backspace => {
                        self.key.pop();
                    }
                    KeyCode::Enter =>{
                        self.key.push_str(cli_clipboard::get_contents().unwrap().as_str());
                    }
                    _ => {}
                }
            }

            _=>{}

        }
        
        Ok(self)
    }

    pub async fn mouse(&mut self, app:&mut App<'_>) -> Result<&mut Self, Box<dyn Error>>{
        
        let event = event::read()?;
        if let Event::Mouse(me) = event{
            if me.kind == MouseEventKind::Down(MouseButton::Left){
                self.mouse = me;
            }
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