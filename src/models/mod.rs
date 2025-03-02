use std::{fmt::Display, io};

use chrono::{DateTime, Local};
use crossterm::event::{self, KeyEventKind};
use ratatui::{prelude::CrosstermBackend, widgets::Widget, Frame, Terminal};
use serde::{Deserialize, Serialize};

use crate::ui::ui;
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
impl  Default for Level {
    fn default() -> Self {
        Self::Easy
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hint {
    pub pattern_rule: String,
    pub hint: String,
}
impl Default for Hint{
    fn default() -> Self {
        Self{
            hint: String::new(),
            pattern_rule:String::new()
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug,Default)]
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
#[derive(Serialize, Deserialize, Clone, Debug,Default)]
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
    pub online:bool
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chat {
    pub sender: String,
    pub reciever: String,
    pub content: String,
    pub created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Battle {
    pub initiator: UserAccount,
    pub reciever: UserAccount,
    pub winner: UserAccount,
    pub pattern: Pattern,
    pub battle_chat: Vec<Chat>,
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
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct App {
    pub redraw: bool,
    pub mode: Modes,
    pub user_account:UserAccount,
    pub hint_toggle: bool,
    pub leaderboard_toggle: bool,
    pub exit: bool,
    pub user_input: String,
}
impl App {
    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f));
        }
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        self.handle_events();
        if self.redraw {
            ui(&self, frame);
        }
        ui(&self, frame);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
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
}

impl Default for App {
    fn default() -> Self {
        Self {
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
