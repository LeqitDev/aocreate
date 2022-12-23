mod input_gatherer;
mod creator;
mod config;

use creator::create_project;
use creator::create_day;
use chrono::{Local, DateTime, Datelike};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let args_length: usize = args.len();
    let _new_arg = String::from("new");
    let _project_arg = String::from("project");

    if args_length > 1 {
        match args[1].as_str() {
            "year" => {
                let mut name = String::from("");
                let mut year: Option<i32> = None;
                if args_length > 3 {
                    name = args[3].clone();
                    year = Some(args[2].parse::<i32>().unwrap());
                } else if args_length > 2 {
                    name = args[2].clone();
                    year = Some(args[2].parse::<i32>().unwrap());
                }
                create_project(name, year);
            },
            "day" => {
                let now: DateTime<Local> = Local::now();
                if args_length > 2 {
                    match args[2].as_str() {
                        "-1" => {
                            create_day(now.day()).await;
                            return;
                        },
                        &_ => {
                            let day: u32 = args[2].parse().unwrap();
                            if day < 25 && day > 0 {
                                create_day(day).await;
                            } else {
                                println!("This day is not a valid day! Please try another day!");
                            }
                            return;
                        },
                    }
                } else {
                    create_day(now.day()).await;
                }
            },
            &_ => {

            }
        }
    }
}
