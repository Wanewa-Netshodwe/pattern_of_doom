mod models;
mod database;
mod ai;
mod ui;
use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use models::App;
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Terminal, Frame,
};
fn main() -> io::Result<()> {
    enable_raw_mode();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let app_result = App::default().run(&mut terminal);
    
    app_result
}
