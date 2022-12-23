use std::{fs, io::Write, path::Path, collections::HashMap, process::Command};
use chrono::{Local, DateTime, Datelike};

use crate::{input_gatherer::{get_input, get_example}, config::{get_config_value, set_config_value_outside}};

// Create project folder and project configuration file
pub fn create_project(name: String, year: Option<i32>) {
    let mut final_name;
    let final_year;
    let now: DateTime<Local> = Local::now();

    // If name and year are not given set name and year to current year
    if name.is_empty() && year == None {
        final_name = format!("{}", now.year());
        final_year = now.year();
    } else {
        final_name = name;
        final_year = year.unwrap();
    }

    // If name starts with a digit place a 'AoC-' before the starting digit because 'cargo init' would throw an error
    if string_starts_with_digit(&final_name) {
        final_name = format!("AoC-{}", final_name);
        println!("The project folder starts with a digit this isn't supported by cargo!\nThe folder was renamed to '{}'", final_name);
    }

    // Create project structure (project folder and src folder inside that)
    fs::create_dir(&final_name).expect("Unable to create directory!");
    fs::create_dir(format!("{}/src", final_name)).expect("Unable to create directory!");

    // Create AoCreate.toml config file
    let config_file_path = format!("{}/AoCreate.toml", &final_name);
    fs::File::create(config_file_path).expect("Error creating file!");

    // Create and write the main.rs file to the content of main.txt (https://github.com/LeqitDev/aocreate/blob/main/src/prefabs/main.txt)
    create_and_write_file(format!("{}/src/main.rs", &final_name), parse_prefab(include_str!("prefabs/main.txt"), Some([("$year", final_year.to_string().as_str())].iter().cloned().collect())));

    // Initialize the rust project
    Command::new("cargo").arg("init").current_dir(format!("./{}", final_name)).spawn().expect("Couldn't initialize rust project!");

    // Add the project year to the AoCreate.toml config
    match set_config_value_outside("year", final_year.to_string().as_str(), format!("{}/AoCreate.toml", final_name).as_str()) {
        Ok(()) => (),
        Err(e) => println!("Error occurred: {}", e),
    }
}


pub async fn create_day(day: u32) {
    // This command has to be executed in the parent project directory
    if !Path::new("AoCreate.toml").exists() {
        println!("AoCreate.toml couldn't be found!");
        return;
    }

    // Get written-out digit of the working day
    let wday = digit_to_string(day.try_into().unwrap());

    // Check if the day already exist
    if Path::new(format!("./{}", wday).as_str()).exists() {
        println!("This day already exists at ./{}!", wday);
        return;
    }

    // Get the project year
    let year = get_config_value("year").unwrap();

    // Create the new day directory
    fs::create_dir(format!("src/{}", wday)).expect("Unable to create directory!");

    // Create and fetch the input data for the day
    let input_txt = get_input(&year, &day.to_string()).await;
    create_and_write_file(format!("src/{}/input.txt", wday), input_txt);

    // Create and fetch the example data for the day
    let example_txt = get_example(&year, &day.to_string()).await;
    create_and_write_file(format!("src/{}/example.txt", wday), example_txt);

    // Create the rust file for the day and fill it with the prefab day.txt (https://github.com/LeqitDev/aocreate/blob/main/src/prefabs/day.txt)
    let day_txt = parse_prefab(include_str!("prefabs/day.txt"), Some([("$day", day.to_string().as_str()), ("$wday", digit_to_string(day.try_into().unwrap()))].iter().cloned().collect()));
    create_and_write_file(format!("src/{}/mod.rs", wday), day_txt);
}



fn create_and_write_file(filepath: String, text: String) {
    // Open/create file at filepath
    let mut file = match fs::OpenOptions::new().create(true).write(true).open(&filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return;
        }
    };

    // Write to that file
    match file.write_all(text.as_bytes()) {
        Ok(_) => println!("Successfully wrote to file ({})!", &filepath),
        Err(e) => println!("Error writing to file: {}", e),
    }
}

fn digit_to_string(digit: u8) -> &'static str {
    // parse digit to written-out digit
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

    // Check that the project config file exists
    if Path::new("AoCreate.toml").exists() {
        // Get the project year
        let year = get_config_value("year").unwrap();

        replacements.insert("$year", &year);

        // set the year in the prefab content
        for (key, value) in replacements {
            replaced_prefab = replaced_prefab.replace(key, value);
        }
    }

    // replace additional values (like $day, $wday)
    match additionals {
        Some(n) => {
            for (k, v) in n {
                replaced_prefab = replaced_prefab.replace(k, v);
            }
        },
        None => (),
    }

    // return the formatted content
    return replaced_prefab;
}

fn string_starts_with_digit(s: &str) -> bool {
    // check if the string starts with a digit (Created by ChatGPT)
    for c in s.chars() {
        match c {
            '0'..='9' => return true, // c is a digit
            _ => return false, // c is not a digit
        }
    }
    false // the string is empty
}