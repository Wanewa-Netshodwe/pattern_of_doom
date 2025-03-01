use std::fmt::Display;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Modes {
    SinglePlayer,
    Battle
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Level{
    Hard,
    Easy,
    Medium,
    Impossible
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hint{
    pub pattern_rule:String,
    pub hint:String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pattern{
    pub pattern:Vec<i32>,
    pub level:Level,
    pub time_taken:u16,
    pub jeopardy:u16,
    pub rule:String,
    pub solved:bool, 
    pub term_to_solve:u32 
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardInfo{
    pub username:String,
    pub rank:String
} 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Leaderboard{
    pub board:Vec<LeaderboardInfo>
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAccount{
   pub username:String,
   pub password:String,
   pub incomplete_pattern:Pattern,
   pub patterns_solved:Vec<Pattern>,
   pub rank:String,
   pub file_path:String,
   pub battles:Vec<Battle>,
   pub battles_won:i32,
   pub points:i32,
   pub hint:Hint
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chat{
    pub sender:String,
    pub reciever:String,
    pub content:String,
    pub created_at:String
    }
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct  Battle{
        pub initiator:UserAccount,
        pub reciever: UserAccount,
        pub winner:UserAccount,
        pub pattern: Pattern,
        pub battle_chat:Vec<Chat>,
        pub active:bool,
        pub punishment_pattern:Vec<i32>,
        pub punishment_pattern_term_to_solve:u32,
        pub punishment_path:String,
        pub punishment_pattern_valid:bool,
        pub level:Level
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

    pub struct App{
        pub redraw:bool,
        pub mode:Modes,
        pub hint_toggle:bool,
        pub leaderboard_toggle:bool

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
    
