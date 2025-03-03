use figlet_rs::FIGfont;
use indicatif::{ProgressBar, ProgressStyle};
use mongodb::bson::Document;
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
};
use std::{process, time};

use crate::{
    database::{
        self, connection, get_connection,
        users::{find_logged_in_user, find_user, login},
    },
    models::{Hint, Pattern, UserAccount},
};

pub async fn signup_login() -> UserAccount {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Pattern Of Doom");
    println!("{}", figure.unwrap());
    loop {
        println!("+------------------------+");
        println!("|  1. Start Game         |");
        println!("|  2. Exit               |");
        println!("+------------------------+");
        print!("input : ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if choice == 1 {
            println!("Connecting Database");
            let connection_complete = Arc::new(AtomicBool::new(false));
            let cc = connection_complete.clone();

            let pb = ProgressBar::new(100);
            pb.set_style(
                ProgressStyle::with_template("{bar:40.white} {percent}% {elapsed} ({eta})")
                    .unwrap()
                    .progress_chars("█▓▒░"),
            );

            let spinner_thread = thread::spawn(move || {
                let mut progress = 0;

                while !cc.load(Ordering::SeqCst) {
                    pb.inc(1);
                    progress += 1;
                    thread::sleep(time::Duration::from_millis(390));
                }
                if progress < 100 {
                    for _ in progress..101 {
                        pb.inc(1);
                        progress += 1;
                        thread::sleep(time::Duration::from_micros(100));
                    }
                }
                pb.finish_with_message("Connected to database!");
            });

            let connection_result = get_connection().await;

            // Signal that the connection is complete
            connection_complete.store(true, Ordering::SeqCst);

            spinner_thread.join().unwrap();
            println!("Connected To Database ");
            let account_creation = Arc::new(AtomicBool::new(false));
            let ac = account_creation.clone();
            let pb2 = ProgressBar::new(100);

            pb2.set_style(
                ProgressStyle::with_template("{spinner:.green} {msg}")
                    .unwrap()
                    .tick_chars("|/-\\"),
            );

            pb2.set_message("Loading Please Wait ...");
            let spinner_thread_names = thread::spawn(move || {
                while !ac.load(Ordering::SeqCst) {
                    thread::sleep(time::Duration::from_millis(100));
                    pb2.tick();
                }
            });
            let mut users: Vec<Document> = Vec::new();

            if let Some(users_docs) = database::get_all_docs().await {
                users = users_docs
            } else {
                let empty_list: Vec<Document> = Vec::new();
                users = empty_list;
            }

            let usernames = database::users::get_all_usernames(&users);
            sleep(Duration::from_secs(4));
            account_creation.store(true, Ordering::SeqCst);
            spinner_thread_names.join().unwrap();

            let mut choice = String::new();
            println!("+------------------------+");
            println!("|  1. Login              |");
            println!("|  2. Create Account     |");
            println!("+------------------------+");
            print!("input : ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut choice)
                .expect("Error reading Line");
            let choice: u64 = choice.trim().parse().expect("eroor parsing");
            match choice {
                1 => {
                    print!("Enter Username : ");
                    io::stdout().flush().unwrap();
                    let mut username = String::new();
                    io::stdin()
                        .read_line(&mut username)
                        .expect("Error reading Line");
                    print!("Enter Password :  ");
                    io::stdout().flush().unwrap();
                    let mut password = String::new();
                    io::stdin()
                        .read_line(&mut password)
                        .expect("Error reading Line");

                    match login(
                        &users,
                        &username.trim().to_string(),
                        &password.trim().to_string(),
                    ) {
                        Err(err) => {
                            println!("{}", err)
                        }
                        Ok(valid) => {
                            let user_account =
                                find_logged_in_user(&users, &username.trim().to_string()).unwrap();
                            return user_account;
                        }
                    }
                }
                2 => {
                    let additional_info = || -> (&Vec<Document>) { (&users) };
                    let account_creation = Arc::new(AtomicBool::new(false));
                    let ac = account_creation.clone();

                    let mut username = String::new();
                    let (user_docs) = additional_info();
                    println!("Enter Username ");
                    let mut username_db = String::new();
                    io::stdin()
                        .read_line(&mut username)
                        .expect("Error reading Line");
                    if let Some(name) = find_user(user_docs, &username) {
                        username_db = name;
                    }
                    let mut valid = usernames.contains(&username.trim().to_string());
                    while valid {
                        username.clear();
                        print!("Username {}", username);
                        print!("is  already taken try another one ");
                        println!("Enter Username ");
                        io::stdin()
                            .read_line(&mut username)
                            .expect("Error reading Line");
                        valid = usernames.contains(&username.trim().to_string());
                    }

                    let mut password = String::new();
                    println!("Enter Password ");
                    io::stdin()
                        .read_line(&mut password)
                        .expect("Error reading Line");
                    println!("Processing");

                    match connection_result {
                        Ok(_) => {
                            let pat = Pattern {
                                jeopardy: 0,
                                solved: true,
                                term_to_solve: 0,
                                time_taken: 0,
                                rule: String::new(),
                                level: crate::models::Level::Easy,
                                pattern: vec![],
                            };
                            let pat_clone = pat.clone();

                            let user = UserAccount {
                                online: true,
                                points: 0,
                                battles: vec![],
                                battles_won: 0,
                                hint: Hint {
                                    hint: String::new(),
                                    pattern_rule: String::new(),
                                },
                                file_path: String::new(),
                                incomplete_pattern: pat_clone,
                                username: username.clone(),
                                password: password.clone(),
                                patterns_solved: vec![pat],
                                rank: String::from("Starterpack"),
                            };
                            let pb = ProgressBar::new(100);

                            pb.set_style(
                                ProgressStyle::with_template("{spinner:.green} {msg}")
                                    .unwrap()
                                    .tick_chars("|/-\\"),
                            );

                            pb.set_message("Loading...");
                            let spinner_thread = thread::spawn(move || {
                                while !ac.load(Ordering::SeqCst) {
                                    thread::sleep(time::Duration::from_millis(100));
                                    pb.tick();
                                }
                            });

                            let existing_user = || -> String { username_db }();

                            let user_doc = database::users::find_user(&users, &existing_user);
                            match user_doc {
                                None => {
                                    database::users::create_user_account(user.clone()).await;
                                    account_creation.store(true, Ordering::SeqCst);
                                    spinner_thread.join().unwrap();
                                    return user;
                                }
                                Some(username) => {
                                    account_creation.store(true, Ordering::SeqCst);
                                    spinner_thread.join().unwrap();
                                    print!("Account Found For ");
                                    print!("{}", username);
                                    print!(" Enter Password To Countinue : ");
                                    let mut password2 = String::new();
                                    io::stdin()
                                        .read_line(&mut password2)
                                        .expect("Error reading Line");
                                    let res = database::users::login(&users, &username, &password2);
                                    match res {
                                        Err(err) => {
                                            println!("{}", err)
                                        }
                                        Ok(valid) => {
                                            let user_account =
                                                find_logged_in_user(&users, &username).unwrap();
                                            return user_account;
                                        }
                                    }
                                }
                            }
                        }

                        Err(e) => println!("An error occurred: {}", e),
                    }
                }
                _ => (),
            }
        } else {
            process::exit(0);
            println!("Database connection rejected");
        }
    }
}
