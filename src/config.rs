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
        let c = Config {
            whitelist: false,
            exclude_explorer: true,
            applications: Vec::new(),
        };
        _save(&c);
        return c;
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(load());
}

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
            let mut toml_str = String::new();
            if file.read_to_string(&mut toml_str).is_err() {
                return Config::default();
            }

            match toml::from_str(&toml_str) {
                Ok(parsed_data) => parsed_data,
                Err(_) => Config::default(),
            }
        }
        Err(_) => Config::default(),
    }
}

fn _save(cfg: &Config) {
    let toml_str = toml::to_string(cfg).unwrap();
    let mut file = File::create("config.toml").unwrap();
    file.write_all(toml_str.as_bytes()).unwrap();
}

pub fn save() {
    _save(&*get());
}
