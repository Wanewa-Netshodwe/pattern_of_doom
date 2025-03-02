mod ai;
mod database;
mod models;
mod signup_login;
mod ui;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};
use models::App;
use ratatui::{
    backend::CrosstermBackend,
    Terminal
};
use signup_login::signup_login;
use std::io::{self, stdout};
#[tokio::main]
async fn main() -> io::Result<()> {
    let user = signup_login().await;
    enable_raw_mode();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App{
        exit:false,
        hint_toggle:false,
        leaderboard_toggle:false,
        redraw:false,
        mode:models::Modes::SinglePlayer,
        user_account:user,
        user_input:String::new()
    };
    let app_result = app.run(&mut terminal);
    app_result
   
}
