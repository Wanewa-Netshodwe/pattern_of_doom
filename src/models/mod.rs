use std::{
    fmt::Display,
    io,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc,
    },
    thread::JoinHandle, vec,
};

use chrono::{DateTime, Local, Utc};
use crossterm::{event::{self, KeyEventKind}, queue};
use futures_util::lock::Mutex;
use mongodb::{
    bson::{doc, to_bson, Document},
    Collection,
};
use rand::Rng;
use ratatui::{prelude::CrosstermBackend, widgets::Widget, Frame, Terminal};
use serde::{Deserialize, Serialize};
use sysinfo::{Disk, Disks};

use crate::{
    ai::{give_hint, is_pattern_valid}, database::cache, game_funtions::{counter, generate_sequence}, ui::ui
};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Modes {
    SinglePlayer,
    Battle,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Level {
    Hard,
    Easy,
    Medium,
    Impossible,
}
impl Default for Level {
    fn default() -> Self {
        Self::Easy
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hint {
    pub pattern_rule: String,
    pub hint: String,
}
impl Default for Hint {
    fn default() -> Self {
        Self {
            hint: String::new(),
            pattern_rule: String::new(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Pattern {
    pub pattern: Vec<i32>,
    pub level: Level,
    pub time_taken: u16,
    pub jeopardy: u16,
    pub rule: String,
    pub solved: bool,
    pub term_to_solve: u32,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardInfo {
    pub username: String,
    pub rank: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Leaderboard {
    pub board: Vec<LeaderboardInfo>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserAccount {
    pub username: String,
    pub password: String,
    pub incomplete_pattern: Pattern,
    pub patterns_solved: Vec<Pattern>,
    pub rank: String,
    pub file_path: String,
    pub battles: Vec<Battle>,
    pub battles_won: i32,
    pub points: i32,
    pub hint: Hint,
    pub online: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub date: String,
    pub time: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Chat {
    pub chats: Vec<Message>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Battle {
    pub initiator: UserAccount,
    pub reciever: UserAccount,
    pub winner: UserAccount,
    pub pattern: Pattern,
    pub battle_chat: Chat,
    pub active: bool,
    pub punishment_pattern: Vec<i32>,
    pub punishment_pattern_term_to_solve: u32,
    pub punishment_path: String,
    pub punishment_pattern_valid: bool,
    pub level: Level,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ReqBody {
    pub contents: Vec<ContentInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentInfo {
    pub parts: Vec<Prompts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prompts {
    pub text: String,
}
#[derive(Debug)]

pub struct App {
    pub chat: Chat,
    pub redraw: bool,
    pub display_messages:Vec<Message>,
    pub mode: Modes,
    pub user_account: UserAccount,
    pub hint_toggle: bool,
    pub leaderboard_toggle: bool,
    pub exit: bool,
    pub user_input: String,
    pub scroll_offset: usize,
    pub scroll_offset_online_users: usize,
}
impl App {
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        chat_collection: &Collection<Document>,
    ) -> io::Result<()> {
        while !self.exit {
            let _ = terminal.draw(|f| {
                let mut sys = Disks::new_with_refreshed_list();
                let mut c_drive: Option<&mut Disk> = Option::None;
                let mut disk_space_avail: Option<u64> = None;
                for disk in &mut sys {
                    if disk.mount_point().display().to_string().starts_with("C:") {
                        c_drive = Some(disk);
                        break;
                    }
                }
                if let Some(disk) = c_drive.as_mut() {
                    disk.refresh();
                    if disk.available_space() > 1_111_741_824 {
                        disk_space_avail = Some(disk.available_space() / 1_073_741_824);
                    } else {
                        disk_space_avail = Some(disk.available_space() / 1_048_576);
                    }
                }
                if self.redraw {
                    ui(&self, f, disk_space_avail.unwrap())
                }
                ui(&self, f, disk_space_avail.unwrap())
            });
            let _ = self.handle_events(chat_collection).await;
        }
        Ok(())
    }

    async fn handle_events(&mut self, chat_collection: &Collection<Document>) -> io::Result<()> {
     
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Up => {
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                            }
                        }
                        event::KeyCode::Down => {
                            let visible_messages = 10;
                            if self.scroll_offset + visible_messages
                                < self.chat.chats.len()
                            {
                                self.scroll_offset += 1;
                            }
                        }
                        event::KeyCode::Enter => {
                            if self.user_input.starts_with("/c") {
                                let now = Utc::now();
                                let message = Message {
                                    content: self.user_input.clone()[3..].to_string(),
                                    sender: self.user_account.username.clone().trim().to_string(),
                                    date: now.date_naive().to_string(),
                                    time: now.time().format("%H:%M:%S").to_string(),
                                };
                                if self.chat.chats.is_empty() {
                                    self.chat.chats.push(message);
                                    if let Ok(messages_bson) = to_bson(&self.chat.chats) {
                                        let _ = chat_collection
                                            .insert_one(
                                                doc! {
                                                    "chats": messages_bson,
                                                    "name": "chats"
                                                },
                                                None,
                                            )
                                            .await;
                                    }
                                    self.redraw = true
                                } else {
                                    let existing_messages = &mut self.chat.chats;
                                    existing_messages.push(message);
                                    if let Ok(messages_bson) = to_bson(existing_messages) {
                                        let _ = chat_collection
                                            .update_one(
                                                doc! { "name": "chats" },
                                                doc! {
                                                    "$set": {
                                                        "chats": messages_bson,
                                                        "name": "chats"
                                                    }
                                                },
                                                None,
                                            )
                                            .await;
                                    }
                                }
                                self.user_input = self.user_input[..3].to_string();
                            }else if self.user_input.starts_with("hint"){
                                let pattern = &self.user_account.incomplete_pattern.pattern;
                                let mut query = String::new();
                                for num in  pattern{
                                    query.push_str(format!("{}",num).as_str());
                                };
                               
                                let res = give_hint(query).await.unwrap();
                                self.user_input = res;

                            }else {
                                //file size calculation
                                let size = Arc::new(AtomicI32::new(0));
                                let size_for_thread = Arc::clone(&size);
                                let path = Path::new(r"C:\Temp\test\file.txt");

                                //time taken calculation
                                let seconds = Arc::new(AtomicI32::new(
                                    self.user_account.incomplete_pattern.time_taken.into(),
                                ));
                                let seconds_clone = Arc::clone(&seconds);
                                let counter_flag = Arc::new(AtomicBool::new(false));
                                if !self.user_account.incomplete_pattern.solved {
                                    counter(seconds, counter_flag);
                                }
                                self.user_input.push_str(
                                    format!("seconds -{}", seconds_clone.load(Ordering::SeqCst))
                                        .as_str(),
                                );
                                if self.user_input.contains("Gen-1") {
                                    if self.user_account.incomplete_pattern.solved { 
                                        let mut rng = rand::thread_rng();
                                        let term_to_solve = rng.gen_range(6..12);
                                        let seq: (Vec<i32>, String) =
                                            generate_sequence(&Level::Easy);
                                        let seq_creation = Arc::new(AtomicBool::new(false));
                                        let sc = seq_creation.clone();
                                        self.user_account.incomplete_pattern.pattern = seq.0;
                                        self.user_account.incomplete_pattern.rule = seq.1;
                                        self.user_account.incomplete_pattern.term_to_solve =
                                            term_to_solve;
                                        self.user_account.incomplete_pattern.level = Level::Easy;
                                        self.user_account.incomplete_pattern.time_taken =
                                            seconds_clone.load(Ordering::SeqCst) as u16;
                                        self.user_account.incomplete_pattern.solved = false;
                                    } else {
                                        self.user_input =
                                            "solve current problem firsttttt".to_string();
                                    }
                                }
                            }
                            self.redraw = true;
                        }
                        event::KeyCode::Char(c) => {
                            if self.user_input.len() < 75 {
                                self.user_input.push(c);
                            } else if self.user_input.len() >= 75 && self.user_input.len() < 95 {
                                self.user_input.push_str("(can not exceed 75 )");
                            } else if self.user_input.len() >= 95 {
                                self.user_input.truncate(75);
                            }
                        }
                        event::KeyCode::Backspace => {
                            self.user_input.pop();
                        }
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn update_messages(&mut self, new_messages: Vec<Message>) {
            self.user_input = "im updating the message".to_string();
            self.display_messages.clear();
            self.display_messages = new_messages;
            self.redraw = true;
    }

    }




impl Default for App {
    fn default() -> Self {
        Self {
            display_messages:Vec::new(),
            scroll_offset: 0,
            scroll_offset_online_users: 0,
            chat: Chat::default(),
            user_account: UserAccount::default(),
            user_input: String::new(),
            exit: false,
            mode: Modes::SinglePlayer,
            hint_toggle: false,
            leaderboard_toggle: false,
            redraw: false,
        }
    }
}
impl Display for UserAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"name : {}\npassword:{}\nrank:{}\nfile_path:{}\nincomplete_patter:{}\npatterns_solved:{:#?}",
        self.username,self.password,self.rank,self.file_path,self.incomplete_pattern,self.patterns_solved
        )
    }
}
impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "general_rule:{}\nlevel:{:?}\npattern:{:?}",
            self.rule, self.level, self.pattern
        )
    }
}
pub enum LoginError {
    Message(String),
}
impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(msg) => write!(f, "Error Occured {}", msg),
        }
    }
}
pub fn generate_app(chat: Chat, user: UserAccount) -> Arc<Mutex<App>> {
    
    let app = Arc::new(Mutex::new( App {
        display_messages:chat.clone().chats,
        scroll_offset: 0,
        scroll_offset_online_users: 0,
        chat: chat,
        exit: false,
        hint_toggle: false,
        leaderboard_toggle: false,
        redraw: false,
        mode: Modes::SinglePlayer,
        user_account: user,
        user_input: String::new(),
    }));

    app
}
