#![allow(dead_code)]
#![allow(unused_imports)]
use std::error::Error;

mod term;
mod ui;

use aranet4::SensorManager;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = term::initialize_console().expect("Could not initialize console");
    let res = term::run_app(&mut terminal).await;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }

    // let sensor = SensorManager::init(None)
    //     .await
    //     .expect("Unable to create sensor manager");

    // let readings = sensor
    //     .read_current_values()
    //     .await
    //     .expect("Could not read current values");
    // println!("{}", readings);
    Ok(())
}
