use std::{
    fs::File,
    io::{Read, Write}, sync::{Mutex, MutexGuard},
};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub whitelist: bool,
    pub exclude_explorer: bool,
    pub applications: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            whitelist: false,
            exclude_explorer: true,
            applications: Vec::from(["thorium".to_string()]),
        }
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(load());
}

// static mut CONFIG: Option<Config> = None;

pub fn get() -> MutexGuard<'static, Config> {
    return CONFIG.lock().unwrap();
}

pub fn exclude_explorer() -> bool {
    get().exclude_explorer
}

pub fn set_exclude_explorer(exclude_explorer: bool) {
    get().exclude_explorer = exclude_explorer;
    save();
}

pub fn whitelisted() -> bool {
    get().whitelist
}

pub fn set_whitelisted(whitelist: bool) {
    get().whitelist = whitelist;
    save();
}

pub fn applications() -> Vec<String> {
    get().applications.clone()
}

pub fn append(application: String) {
    get().applications.push(application);
    save();
}

pub fn remove(application: String) {
    get().applications.retain(|x| x != &application);
    save();
}
pub fn load() -> Config {
    let file_path = "config.toml";

    match File::open(file_path) {
        Ok(mut file) => {
            // Read the file content into a string
            let mut toml_str = String::new();
            if file.read_to_string(&mut toml_str).is_err() {
                // Handle read error and return default value
                return Config::default();
            }

            // Parse the TOML string into a Config struct
            match toml::from_str(&toml_str) {
                Ok(parsed_data) => parsed_data,
                Err(_) => Config::default(),
            }
        }
        Err(_) => Config::default(),
    }
}

pub fn save() {
    let cfg = get();
    let toml_str = toml::to_string(&*cfg).unwrap();
    let mut file = File::create("config.toml").unwrap();
    file.write_all(toml_str.as_bytes()).unwrap();
}
