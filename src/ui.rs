use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Gauge},
    Frame,
};
use tui_logger::TuiLoggerWidget;

pub fn draw<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
        .split(f.size());
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(chunks[0]);

    draw_graph(f, middle_chunks[0]);
    draw_current_readings(f, middle_chunks[1], app);
    draw_status(f, chunks[1]);
}

fn draw_status<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Logs",
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    ));
    let logs = TuiLoggerWidget::default().block(block);
    f.render_widget(logs, area);
}

fn draw_graph<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default()
        .title("Historical Data")
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_current_readings<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let readings = app.get_cached_readings();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(area);

    let co2_label = Span::styled(
        format!("{:.2}ppm", readings.co2_level()),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let co2_gauge = Gauge::default()
        .block(Block::default().title("CO2 Level").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(readings.co2_level() as f64 / 5000_f64)
        .label(co2_label)
        .use_unicode(true);

    let temp_label = Span::styled(
        format!("{:.2}°F", readings.temperature()),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let temp_gauge = Gauge::default()
        .block(Block::default().title("Temperature").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .ratio(readings.temperature() as f64 / 200_f64)
        .label(temp_label)
        .use_unicode(true);

    let pressure_label = Span::styled(
        format!("{:.2}hPa", readings.pressure()),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let pressure_gauge = Gauge::default()
        .block(Block::default().title("Pressure").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(readings.pressure() as f64 / 2000_f64)
        .label(pressure_label)
        .use_unicode(true);

    let humidity_label = Span::styled(
        format!("{:.2}%", readings.humidity()),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let humidity_gauge = Gauge::default()
        .block(Block::default().title("Humidity").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(readings.humidity() as f64 / 100_f64)
        .label(humidity_label)
        .use_unicode(true);

    f.render_widget(co2_gauge, chunks[0]);
    f.render_widget(temp_gauge, chunks[1]);
    f.render_widget(pressure_gauge, chunks[2]);
    f.render_widget(humidity_gauge, chunks[3]);
}
