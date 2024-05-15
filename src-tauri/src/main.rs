// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use models::GeorgState;

mod geo;
mod models;
mod parse;

fn main() {
    tauri::Builder::default()
         .manage(GeorgState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
