mod ai;
mod database;
mod game_funtions;
mod models;
mod signup_login;
mod ui;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use database::{
    cache::{self, Cache, CHATCOLLECTION},
    get_chats,
};
use futures_util::{lock::Mutex, TryStreamExt};
use models::{generate_app, App, Chat};
use mongodb::{bson::{doc, from_bson, Bson, Document}, options::{ChangeStreamOptions, FullDocumentType}, Collection};
use ratatui::{backend::CrosstermBackend, Terminal};
use signup_login::signup_login;
use std::{io::{self, stdout}, sync::Arc};
use sysinfo::Disks;

async fn setup_change_stream(collection: Collection<Document>,app:Arc<Mutex<App>>) {
    println!("Setting up change stream...");
    tokio::spawn(async move {
        let pipeline = vec![doc! {
            "$match": {
                "operationType": { "$in": ["insert", "update"] },
                "fullDocument.name": "chats"
            }
        }];

        let options = ChangeStreamOptions::builder()
            .full_document(Some(FullDocumentType::UpdateLookup))
            .build();

        let mut change_stream = match collection.watch(pipeline, Some(options)).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Error setting up change stream: {}", e);
                return;
            }
        };

        while let Ok(Some(change)) = change_stream.try_next().await {
            println!("change happed...");
            if let Some(full_doc) = change.full_document {
                match from_bson::<Chat>(Bson::Document(full_doc)) {
                    Ok(data) => {
                        // println!("found {:?}",data.chats);
                        println!("trying to get lock ");
                        let mut  app_lock = app.lock().await;
                        println!("lock aquired");
                        app_lock.user_input = "im change stream happend".to_string();
                        app_lock.update_messages(data.chats).await;
                        if app_lock.chat.chats.len() >= 10
                            && app_lock.scroll_offset + 10 >= app_lock.chat.chats.len() - 2
                        {
                            app_lock.scroll_offset = app_lock.chat.chats.len() - 10;
                        }
                        app_lock.redraw = true; 
                        
                    }
                    Err(e) => eprintln!("Error deserializing document: {}", e),
                }
            }
        }
    });
}



#[tokio::main]
async fn main() -> io::Result<()> {
    let user = signup_login().await;
    let _ = enable_raw_mode();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let chats = get_chats().await;
    let cache = cache::GLOBAL_CACHE.lock().await;
    let chat_collection = cache.get_collection(CHATCOLLECTION.to_string()).unwrap();
    
    match chats {
        Some(mess) => {
            println!("Trying to lock app in main...");
            let chat = Chat { chats: mess };
            let  app = generate_app(chat, user);
            setup_change_stream(chat_collection.clone(),Arc::clone(&app)).await;
            let mut app_lock = app.lock().await;
            let app_result = app_lock.run(&mut terminal, chat_collection).await;
            app_result
        }
        None => {
            let chat = Chat { chats: vec![] };
            let  app = generate_app(chat, user);
            let mut app_lock = app.lock().await;
            setup_change_stream(chat_collection.clone(),Arc::clone(&app)).await;
            let app_result = app_lock.run(&mut terminal, chat_collection).await;
            app_result
        }
    }
}
