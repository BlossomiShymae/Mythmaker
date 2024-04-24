// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod client;
pub mod types;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            client::get_local_player_challenges,
            client::get_local_player_summary,
            client::get_local_player_categories,
            client::get_current_summoner
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
