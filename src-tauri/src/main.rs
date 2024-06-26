// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use models::AppState;

mod geo;
mod models;
mod parse;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            parse::import_candidate_data,
            parse::import_assistant_data,
            parse::candidates::get_candidate_data,
            parse::assistance::get_assistant_data,
            geo::find_candidate_matches,
            geo::find_childcare_req_matches,
            geo::find_assistant_matches,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
