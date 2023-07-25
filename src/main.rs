use std::{error::Error, io::{Stdout, self, stdout}, time::Duration, process, fs};

use async_openai::Client;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture}, execute};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};

mod app;
use app::{io::{Input, Output, InputMode}, App};
mod ui;
use ui::Ui;
mod ai;
use ai::Ai;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    
    let mut terminal = App::setup_terminal()?;
    app.run(&mut terminal).await?;
    App::restore_terminal(&mut terminal)?;

    Ok(())
}


