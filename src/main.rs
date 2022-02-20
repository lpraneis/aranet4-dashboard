#![allow(dead_code)]
#![allow(unused_imports)]
use argh::FromArgs;
use std::error::Error;

mod app;
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

/// Aranet4 Sensor Dashboard
#[derive(Debug, FromArgs)]
struct Cli {
    /// an optional Bluetooth Address for the Sensor. If not provided, the bluetooth peripherals
    /// will be searched for a name containing "Aranet4"
    #[argh(option)]
    address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    term::run(cli.address).await?;
    Ok(())
}
