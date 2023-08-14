pub mod io;
use io::InputMode;
use std::{fs, io::{Stdout, stdout}, error::Error, time::Duration};

use async_openai::Client;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, ExecutableCommand, execute, event::{EnableMouseCapture, DisableMouseCapture, self, Event, KeyCode}};
use io::{Input, Output};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{ui::Ui, ai::Ai};


#[derive(Clone)]
pub struct App{
    pub client : Client,
    pub ui: Ui,
    pub ai: Ai,
    pub input: Input,
    pub output: Output,
    pub running: bool,
}

impl App{
    pub fn new() -> Self{
        let api_key = fs::read_to_string("logs/key.txt").unwrap();
        let client = Client::new().with_api_key(api_key);
        
        let mut input_binding = Input::new();
        let input = input_binding;

        let mut output_binding = Output::new();
        let output = output_binding
            .logs_path("logs/chat.txt".to_owned());

        let mut ai_binding = Ai::new(client.clone());
        let ai = ai_binding
            .system(fs::read_to_string("logs/system.txt").unwrap())
            .assistant("".to_owned());
        

        Self{
            client: client,
            ui: Ui::new(),
            ai:ai.clone(),
            input: input,
            output: output,
            running : true,
        }
    }

    pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        execute!(stdout, EnableMouseCapture)?;
        Ok(Terminal::new(CrosstermBackend::new(stdout))?)
    }

    pub fn restore_terminal(
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        execute!(terminal.backend_mut(), DisableMouseCapture)?;
        Ok(terminal.show_cursor()?)
    }

    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>>{

        while self.running {
            terminal.draw(|frame| Ui::draw(frame, self))?;
            
            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => {
                        //pass key to input
                        self.input.key(&mut self.clone(), key).await?;
                        
                        //if q key is pressed quit program
                        match self.input.mode{   
                            InputMode::Normal => {
                                match key.code {
                                    KeyCode::Char('q') => {
                                        self.running = false;
                                    }
                                    _ => {}
                                }
                            }, 
                            _ => {}
                        }
                    }
                    Event::Mouse(me) => {
                        self.input.mouse = me;
                    }
                    _ => {}
                }
                //self.input.mouse(&mut self.clone()).await?;
            }
        }

        Ok(())
    }
}
