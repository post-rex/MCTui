mod constants;
mod utils;
mod structs;
mod mctui;

use std::path::Path;
use utils::*;
use structs::settings;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fs::create_dir_all;
use crate::mctui::tui::start_tui;

lazy_static! {
    static ref SETTINGS: Mutex<settings::Settings> = Mutex::new(settings::Settings::new().unwrap());
    static ref CONNECTION: Mutex<bool> = Mutex::new(false);
}

fn main() {
    let mut dot = format!("{}/.mctui", std::env::var("HOME").unwrap());
    match std::env::var("DOT_MCTUI") {
        Ok(val) => dot = val,
        Err(_) => std::env::set_var("DOT_MCTUI", dot.to_owned())
    }

    create_dir_all(dot.to_owned()).unwrap();
    std::env::set_current_dir(Path::new(&dot)).unwrap();
    universal::start_checker();
    start_tui().expect("Error occurred");
}
