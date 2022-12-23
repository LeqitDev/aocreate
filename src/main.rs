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

    // process the command-line args
    if args_length > 1 {
        match args[1].as_str() {
            // Create new project
            "year" => {
                let mut name = String::from("");
                let mut year: Option<i32> = None;

                // year [year(2015-now)] [name]
                // year [year(2015-now)]            --> year year year
                // year                             --> year current_year current_year

                // TODO: clamp the possible year

                if args_length > 3 {
                    name = args[3].clone();
                    year = Some(args[2].parse::<i32>().unwrap());
                } else if args_length > 2 {
                    name = args[2].clone();
                    year = Some(args[2].parse::<i32>().unwrap());
                }

                create_project(name, year);
            },
            // Create new day in the project
            "day" => {
                let now: DateTime<Local> = Local::now();

                // day [day(1-25)]
                // day              --> day current_day

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
            &_ => (),
        }
    }
}
