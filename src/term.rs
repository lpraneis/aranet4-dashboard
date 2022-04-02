use crate::{app::App, handler::IoEvent, ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::sync::Arc;
use std::time::Duration;
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

pub async fn run(app: &Arc<tokio::sync::Mutex<App>>) -> io::Result<()>
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
async fn run_app<B>(
    terminal: &mut Terminal<B>,
    app: &Arc<tokio::sync::Mutex<App>>,
) -> io::Result<()>
where
    B: Backend,
{
    // calculate how long to poll for keyboard input
    let timeout = Duration::from_millis(250);

    // initialize the app
    {
        let mut app = app.lock().await;
        app.dispatch(IoEvent::Connect).await;
        app.dispatch(IoEvent::GetCurrentData).await;
    }

    // TODO
    // - Update the values on a timer ( 5 min )
    loop {
        {
            let app = app.lock().await;
            terminal.draw(|f| ui::draw(f, &app))?;
        }

        // poll for keyboard input
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}
