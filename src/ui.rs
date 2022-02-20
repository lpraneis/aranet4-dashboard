#![allow(unused)]
use crate::app::App;
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(chunks[1]);

    // let block = Block::default().title("Graph").borders(Borders::ALL);
    // f.render_widget(block, chunks[1]);
    draw_titlebar(f, chunks[0]);
    draw_graph(f, middle_chunks[0]);
    draw_current_readings(f, middle_chunks[1]);
    draw_status(f, chunks[2]);
}

fn draw_status<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Status",
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(Span::from("Connecting to adaptor..."))
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_graph<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, area);
}
fn draw_titlebar<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, area);
}
fn draw_current_readings<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, area);
}
