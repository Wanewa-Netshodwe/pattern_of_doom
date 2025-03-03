use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use rand::Rng;

use crate::models::{App, Level};

pub fn generate_sequence(level: &Level) -> (Vec<i32>, String) {
    let mut rng = rand::thread_rng();

    match level {
        Level::Medium => {
            let mut pattern = Vec::new();
            let n = rng.gen_range(6..21);
            let operand_num = rng.gen_range(1..6);
            let complementary_num = rng.gen_range(1..65);
            let mut value = n;

            for _ in 0..4 {
                value = match operand_num {
                    1 => value + complementary_num,
                    2 => value - complementary_num,
                    3 => value * rng.gen_range(2..5),
                    4 => value / rng.gen_range(2..5).max(1),
                    5 => value * 2,
                    _ => unreachable!(),
                };
                pattern.push(value);
            }

            let rule = format!(
                "{},{},{}",
                n,
                operand_num_to_string(operand_num),
                complementary_num
            );
            (pattern, rule)
        }

        Level::Easy => {
            let mut pattern = Vec::new();
            let n = rng.gen_range(2..21);
            let operand_num = rng.gen_range(1..5);
            let complementary_num = rng.gen_range(1..25);
            let mut value = n;

            for _ in 0..4 {
                value = match operand_num {
                    1 => value + complementary_num,
                    2 => value - complementary_num,
                    3 => value * rng.gen_range(1..4),
                    4 => value * 2,
                    _ => unreachable!(),
                };
                pattern.push(value);
            }

            let rule = format!(
                "{},{},{}",
                n,
                operand_num_to_string(operand_num),
                complementary_num
            );
            (pattern, rule)
        }

        Level::Hard => {
            let mut pattern = Vec::new();
            let n = rng.gen_range(10..50);
            let operand_num = rng.gen_range(1..7);
            let complementary_num = rng.gen_range(2..100);
            let mut value: i32 = n;

            for i in 0..4 {
                value = match operand_num {
                    1 => value + complementary_num,
                    2 => value - complementary_num,
                    3 => value * rng.gen_range(2..7),
                    4 => value / rng.gen_range(2..5).max(1),
                    5 => value.pow(rng.gen_range(1..3)),
                    6 => fibonacci(i + 5),
                    _ => unreachable!(),
                };
                pattern.push(value);
            }

            let rule = format!(
                "{},{},{}",
                n,
                operand_num_to_string(operand_num),
                complementary_num
            );
            (pattern, rule)
        }

        Level::Impossible => {
            let mut pattern = Vec::new();
            let n = rng.gen_range(50..200);
            let operand_num = rng.gen_range(1..8);
            let complementary_num = rng.gen_range(5..150);
            let mut value: i32 = n;

            for i in 0..4 {
                value = match operand_num {
                    1 => value + complementary_num,
                    2 => value - complementary_num,
                    3 => value * rng.gen_range(2..10),
                    4 => value / rng.gen_range(2..10).max(1),
                    5 => value.pow(rng.gen_range(1..3)),
                    6 => (value as f64).sqrt().round() as i32,
                    7 => fibonacci(i + 8),
                    _ => unreachable!(),
                };
                pattern.push(value);
            }

            let rule = format!(
                "{},{},{}",
                n,
                operand_num_to_string(operand_num),
                complementary_num
            );
            (pattern, rule)
        }
    }
}

fn fibonacci(n: usize) -> i32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

fn operand_num_to_string(num: i32) -> &'static str {
    match num {
        1 => "+",
        2 => "-",
        3 => "*",
        4 => "/",
        5 => "^",
        6 => "√",
        7 => "Fibonacci",
        _ => "?",
    }
}

pub fn counter(seconds: Arc<AtomicI32>, flag_clone: Arc<AtomicBool>) {
    let counter_handle = thread::spawn(move || {
        while !flag_clone.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
            seconds.fetch_add(1, Ordering::SeqCst);
        }
    });
}
