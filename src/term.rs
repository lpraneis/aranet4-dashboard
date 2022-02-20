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

pub async fn run(address: Option<String>) -> io::Result<()>
where
{
    let mut terminal = initialize_console().expect("Could not initialize console");
    let app = App::new(address);
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
async fn run_app<B>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()>
where
    B: Backend,
{
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
        if last_tick.elapsed() >= Duration::from_secs(10) {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.status() != ConnectionStatus::Connected {
            app.init().await;
        }
    }
}
