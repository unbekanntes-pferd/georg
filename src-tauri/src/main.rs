// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use models::AppState;

mod geo;
mod models;
mod parse;
mod db;

#[tokio::main]
async fn main() {

    let pool = db::init_database().await.expect("error while initializing database");
    let geo_repository = Arc::new(db::SqliteGeoCodeRepository::new(pool));

    
    tauri::Builder::default()
        .manage(AppState::new(geo_repository))
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
