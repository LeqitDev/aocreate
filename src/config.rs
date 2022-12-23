use std::{path::Path, collections::HashMap, fs, io::Write};
use config::{Config, ConfigError, File, FileFormat};

pub fn get_config_value(key: &str) -> Result<String, ConfigError> {
    let path;

    if Path::new("AoCreate.toml").exists() {
        path = "AoCreate.toml";
    } else if Path::new("../AoCreate.toml").exists() {
        path = "AoCreate.toml";
    } else {
        return Err(ConfigError::NotFound("AoCreate.toml not found. Please try this in the project root directory!".to_string()))
    }

    let builder = Config::builder()
        .add_source(File::new(path, FileFormat::Toml));
    
    match builder.build() {
        Ok(config) => {
            return config.get(key);
        },
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn set_config_value(key: &str, value: &str) -> Result<(), ConfigError>  {
    let path;

    if Path::new("AoCreate.toml").exists() {
        path = "AoCreate.toml";
    } else if Path::new("../AoCreate.toml").exists() {
        path = "AoCreate.toml";
    } else {
        return Err(ConfigError::NotFound("AoCreate.toml not found. Please try this in the project root directory!".to_string()))
    }

    return _set_config_value_raw(key, value, path);
}

pub fn set_config_value_outside(key: &str, value: &str, path: &str) -> Result<(), ConfigError>  {
    return _set_config_value_raw(key, value, path);
}

pub fn _set_config_value_raw(key: &str, value: &str, path: &str) -> Result<(), ConfigError> {

    let builder = Config::builder()
        .add_source(File::new(path, FileFormat::Toml))
        .set_override(key, value).unwrap();
    
    match builder.build() {
        Ok(_config) => {
            let mut config_file = match fs::OpenOptions::new().create(true).write(true).open(path) {
                Ok(file) => file,
                Err(e) => {
                    println!("Error creating file: {}", e);
                    return Err(ConfigError::NotFound("AoCreate.toml not found. Please try this in the project root directory!".to_string()));
                }
            };

            let toml_config = toml::to_string(&_config.try_deserialize::<HashMap<String, String>>().unwrap()).unwrap();

            match config_file.write_all(toml_config.as_bytes()) {
                Ok(_) => println!("Successfully wrote to file!"),
                Err(e) => println!("Error writing to file: {}", e),
            }

            return Ok(());
        },
        Err(e) => {
            return Err(e)
        }
    }
}