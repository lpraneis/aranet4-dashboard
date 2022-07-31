use argh::FromArgs;
use handler::run_handler;
use std::error::Error;
use std::sync::Arc;

mod app;
mod handler;
mod term;
mod ui;

use app::App;
use log::LevelFilter;

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

    tui_logger::init_logger(LevelFilter::Info).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Info);

    let (sync_io_tx, sync_io_rx) = tokio::sync::mpsc::channel::<handler::IoEvent>(100);
    let app = Arc::new(tokio::sync::Mutex::new(App::new(
        sync_io_tx.clone(),
        cli.address,
    )));
    let app_ui = Arc::clone(&app);
    // run the handler in a new thread
    tokio::spawn(run_handler(app, sync_io_rx));

    // run the terminal app
    term::run(&app_ui).await?;
    Ok(())
}
