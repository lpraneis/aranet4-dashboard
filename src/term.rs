use crate::{
    app::{App, ConnectionStatus},
    ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::{Duration, Instant};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn initialize_console() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub async fn run(app: &mut App) -> io::Result<()>
where
{
    let mut terminal = initialize_console().expect("Could not initialize console");
    let res = run_app(&mut terminal, app).await;
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
    Ok(())
}
async fn run_app<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B: Backend,
{
    let tick_rate = Duration::from_secs(5);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Try and connect to the app if we're not connected
        if app.status() != ConnectionStatus::Connected {
            eprintln!("Trying to connect");
            app.init().await;
            app.update_cache().await;
            eprintln!("Connected...");
        }
        // calculate how long to poll for keyboard input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // poll for keyboard input
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        // call app on tick if needed
        if last_tick.elapsed() >= tick_rate {
            app.on_tick().await;
            last_tick = Instant::now();
        }
    }
}
