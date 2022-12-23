use std::{fs, io::Write, path::Path, collections::HashMap, process::Command};
use chrono::{Local, DateTime, Datelike};

use crate::{input_gatherer::{get_input, get_example}, config::{get_config_value, set_config_value_outside}};

// Create project folder and project configuration file
pub fn create_project(name: String, year: Option<i32>) {
    let mut final_name;
    let final_year;
    let now: DateTime<Local> = Local::now();

    // If name is not given set name to current year
    if name.is_empty() && year == None {
        final_name = format!("{}", now.year());
        final_year = now.year();
    } else {
        final_name = name;
        final_year = year.unwrap();
    }

    if string_starts_with_digit(&final_name) {
        final_name = format!("AoC-{}", final_name);
        println!("The project folder starts with a digit this isn't supported by cargo!\nThe folder was renamed to '{}'", final_name);
    }

    fs::create_dir(&final_name).expect("Unable to create directory!");
    fs::create_dir(format!("{}/src", final_name)).expect("Unable to create directory!");

    let config_file_path = format!("{}/AoCreate.toml", &final_name);
    fs::File::create(config_file_path).expect("Error creating file!");

    create_and_write_file(format!("{}/src/main.rs", &final_name), parse_prefab(include_str!("prefabs/main.txt"), Some([("$year", final_year.to_string().as_str())].iter().cloned().collect())));

    Command::new("cargo").arg("init").current_dir(format!("./{}", final_name)).spawn().expect("Couldn't initialize rust project!");
    /* let main_file_path = format!("{}/main.rs", &final_name);
    let mut main = match fs::OpenOptions::new().create(true).write(true).open(main_file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return;
        }
    };

    // let main_fn = format!("/*\n * Welcome to your Advent of Code project\n *\n * Checkout the Challenges at https://adventofcode.com/{}.\n *\n * This project was created with AoCreate (https://github.com/LeqitDev/aocreate) \n * by CubeCoder \n */ \n\n fn main() {{ \n\t\n }}\n", now.year());
    let main_fn = include_str!("prefabs/main.txt");

    match main.write_all(parse_prefab(main_fn, None).as_bytes()) {
        Ok(_) => println!("Successfully wrote to file!"),
        Err(e) => println!("Error writing to file: {}", e),
    } */

    match set_config_value_outside("year", final_year.to_string().as_str(), format!("{}/AoCreate.toml", final_name).as_str()) {
        Ok(()) => (),
        Err(e) => println!("Error occurred: {}", e),
    }
}


pub async fn create_day(day: u32) {
    if !Path::new("AoCreate.toml").exists() {
        println!("AoCreate.toml couldn't be found!");
        return;
    }

    let wday = digit_to_string(day.try_into().unwrap());

    if Path::new(format!("./{}", wday).as_str()).exists() {
        println!("This day already exists at ./{}!", wday);
        return;
    }

    let year = get_config_value("year").unwrap();

    fs::create_dir(format!("src/{}", wday)).expect("Unable to create directory!");

    let input_txt = get_input(&year, &day.to_string()).await;
    create_and_write_file(format!("src/{}/input.txt", wday), input_txt);

    let example_txt = get_example(&year, &day.to_string()).await;
    create_and_write_file(format!("src/{}/example.txt", wday), example_txt);

    let day_txt = parse_prefab(include_str!("prefabs/day.txt"), Some([("$day", day.to_string().as_str()), ("$wday", digit_to_string(day.try_into().unwrap()))].iter().cloned().collect()));
    create_and_write_file(format!("src/{}/mod.rs", wday), day_txt);

    /* let mut days = match get_config_value("days") {
        Ok(days) => days,
        Err(_e) => "".to_string(),
    };

    if days.is_empty() {
        days = format!("[{}]", day);
    } else {
        days = days.replace("]", format!(",{}]", day).as_str());
    }

    match set_config_value("days", days.as_str()) {
        Ok(()) => (),
        Err(e) => println!("Error occurred: {}", e),
    } */
}



fn create_and_write_file(filepath: String, text: String) {
    // let input_file_path = format!("{}/input.txt", day);
    let mut file = match fs::OpenOptions::new().create(true).write(true).open(&filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return;
        }
    };
        
    // let input_txt = get_input("2022", &day.to_string()).await;

    match file.write_all(text.as_bytes()) {
        Ok(_) => println!("Successfully wrote to file ({})!", &filepath),
        Err(e) => println!("Error writing to file: {}", e),
    }
}

fn digit_to_string(digit: u8) -> &'static str {
    match digit {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        21 => "twenty_one",
        22 => "twenty_two",
        23 => "twenty_three",
        24 => "twenty_four",
        25 => "twenty_five",
        _ => "Invalid",
    }
}

fn parse_prefab(prefab: &str, additionals: Option<HashMap<&str, &str>>) -> String {
    let mut replacements: HashMap<&str, &str> = HashMap::new();
    let mut replaced_prefab: String = prefab.to_string();

    if Path::new("AoCreate.toml").exists() {
        let year = get_config_value("year").unwrap();

        replacements.insert("$year", &year);
        // replacements.insert("// $begin", "");
        // replacements.insert("// $end", "");
        for (key, value) in replacements {
            replaced_prefab = replaced_prefab.replace(key, value);
        }
    }

    match additionals {
        Some(n) => {
            for (k, v) in n {
                replaced_prefab = replaced_prefab.replace(k, v);
            }
        },
        None => (),
    }

    return replaced_prefab;
}

fn string_starts_with_digit(s: &str) -> bool {
    for c in s.chars() {
        match c {
            '0'..='9' => return true, // c is a digit
            _ => return false, // c is not a digit
        }
    }
    false // the string is empty
}