mod ai;
mod database;
mod models;
mod signup_login;
mod ui;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};
use database::{cache::{self, Cache, CHATCOLLECTION}, get_chats};
use models::{App, Chat};
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
    let chats = get_chats().await;
    let cache = cache::GLOBAL_CACHE.lock().await;
    let chat_collection = cache.get_collection(CHATCOLLECTION.to_string()).unwrap();
    match chats {
        Some(mess)=>{
            let chat =Chat{
                chats : mess
            };
            let mut app = App{
                scroll_offset:0,
                scroll_offset_online_users:0,
                chat:chat,
                exit:false,
                hint_toggle:false,
                leaderboard_toggle:false,
                redraw:false,
                mode:models::Modes::SinglePlayer,
                user_account:user,
                user_input:String::new()
            };
            let app_result = app.run(&mut terminal,chat_collection).await;
            app_result
        }
        None=>{
            println!("no chats found");
            let chat =Chat{
                chats:vec![]
            };
            let mut app = App{
                scroll_offset:0,
                scroll_offset_online_users:0,
                chat:chat,
                exit:false,
                hint_toggle:false,
                leaderboard_toggle:false,
                redraw:false,
                mode:models::Modes::SinglePlayer,
                user_account:user,
                user_input:String::new()
            };
            let app_result = app.run(&mut terminal,chat_collection);
            app_result.await
        }
    }
  

    
   
}
