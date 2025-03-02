use super::cache;
use crate::models::{Battle, Hint, LoginError, Pattern, UserAccount};
use mongodb::{
    bson::{self, doc, from_bson, Bson, Document},
    Collection,
};
use serde::Serialize;
pub async fn create_user_account(user_details: UserAccount) {
    let doc = doc! {
        "online":&user_details.online,
        "username": &user_details.username,
        "rank": &user_details.rank,
        "file_path": &user_details.file_path,
        "incomplete_pattern": to_bson(&user_details.incomplete_pattern),
        "patterns_solved": to_bson(&user_details.patterns_solved),
        "password": &user_details.password,
        "hint":to_bson(&user_details.hint),
        "battles":to_bson(&user_details.battles),
        "battles_won":&user_details.battles_won,
        "points":&user_details.points
    };
    let cache = cache::GLOBAL_CACHE.lock().await;
    let col = cache.get_collection("UserAccounts".to_string()).unwrap();
    save_document(&Ok(col), &doc).await;
}
pub async fn update_user_account(user_details: UserAccount) {
    let filter = doc! { "username": &user_details.username };

    let update = doc! {
        "$set": {
            "file_path": &user_details.file_path,
            "incomplete_pattern":to_bson(&user_details.incomplete_pattern),
            "rank": &user_details.rank,
            "patterns_solved": to_bson(&user_details.patterns_solved),
        }
    };

    let cache = cache::GLOBAL_CACHE.lock().await;
    if !cache.is_empty() {
        let collection = cache.get_collection("UserAccounts".to_string());
        if let Some(col) = collection {
            let res = col.update_one(filter, update, None).await;
            match res {
                Ok(_) => {
                    //  println!("Successfully updated the document.");
                    // println!(".");
                }
                Err(_) => {
                    println!("No matching document found.");
                }
            }
        }
    } else {
        println!("cache is empty cant save ")
    }
}

fn formatter(value: &str, user: &Document) -> String {
    user.get(value)
        .unwrap()
        .to_string()
        .replace("\"", "")
        .trim()
        .to_string()
}

pub fn find_user(users: &Vec<Document>, username: &String) -> Option<String> {
    for user in users {
        let doc_username = user
            .get("username")
            .unwrap()
            .to_string()
            .replace("\"", "")
            .trim()
            .to_string();

        if doc_username.eq(username) {
            return Some(doc_username);
        }
    }
    return None;
}
pub fn find_logged_in_user(users: &Vec<Document>, username: &String) -> Option<UserAccount> {
    for user in users {
        let hint = match from_bson::<Hint>(user.get("hints").unwrap().clone()) {
            Ok(data) => data,
            Err(e) => {
                panic!("error {}", e)
            }
        };

        let doc_username = user
            .get("username")
            .unwrap()
            .to_string()
            .replace("\"", "")
            .trim()
            .to_string();
        if doc_username.eq(username) {
            let mut user_account = UserAccount {
                online:user.get("online").unwrap().as_bool().unwrap(),
                password: formatter("password", user),
                file_path: formatter("file_path", user),
                incomplete_pattern: match user.get("incomplete_pattern") {
                    Some(val) => match from_bson::<Pattern>(val.clone()) {
                        Ok(val) => val,
                        Err(err) => {
                            eprintln!("Failed to parse PatternInfo: {}", err);
                            continue;
                        }
                    },
                    None => {
                        continue;
                    }
                },
                username: formatter("name", user),
                rank: formatter("rank", user),
                battles: user
                    .get("battles")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|item| match from_bson::<Battle>(item.clone()) {
                        Ok(pattern_info) => Some(pattern_info),
                        Err(e) => {
                            eprintln!("Failed to parse PatternInfo: {}", e);
                            None
                        }
                    })
                    .collect(),
                hint: hint,
                battles_won: user.get("battles_won").unwrap().as_i32().unwrap(),
                points: user.get("points").unwrap().as_i32().unwrap(),
                patterns_solved: user
                    .get("patterns_solved")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|item| match from_bson::<Pattern>(item.clone()) {
                        Ok(pattern_info) => Some(pattern_info),
                        Err(e) => {
                            eprintln!("Failed to parse PatternInfo: {}", e);
                            None
                        }
                    })
                    .collect(),
            };
            return Some(user_account);
        }
    }
    None
}
pub fn login(
    users: &Vec<Document>,
    username: &String,
    password: &String,
) -> Result<bool, LoginError> {
    for user in users {
        // println!("comapring {} with {}", formatter("name", user), username);
        // println!(
        //     "comapring {} password with {} password",
        //     formatter("password", user),
        //     password
        // );
        if formatter("username", user) == (*username) {
            if formatter("password", user) == (*password) {
                return Ok(true);
            }
        }
    }

    return Err(LoginError::Message("Incorrect Password".to_string()));
}

pub fn get_all_usernames(docs: &Vec<Document>) -> Vec<String> {
    if docs.is_empty() {
        let empty_list: Vec<String> = Vec::new();
        println!("user emptys");
        return empty_list;
    }

    let usernames: Vec<String> = docs
        .iter()
        .filter_map(|doc| doc.get("username"))
        .map(|name| name.to_string().replace("\"", "").trim().to_string())
        .collect();

    usernames
}

fn to_bson<T>(value: &T) -> Bson
where
    T: Serialize,
{
    bson::to_bson(value).expect("Failed to convert to BSON")
}

async fn save_document(
    collection: &Result<&Collection<Document>, mongodb::error::Error>,
    doc: &Document,
) {
    match collection {
        Ok(col) => {
            if let Err(error) = col.insert_one(doc, None).await {
                eprintln!("Error inserting document: {}", error);
            } else {
                println!("Account Created");
            }
        }
        Err(error) => {
            println!("Error retrieving collection: {}", error);
        }
    }
}
