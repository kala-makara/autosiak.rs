#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod unit;

use unit::Session;
use unit::login_handler::{login, logout};
use unit::worker_handler::worker;

use std::sync::Arc;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use tauri::Manager;

#[tauri::command]
fn credit() {
    open::that("https://github.com/deadManAlive").unwrap_or(());
}


fn main() {
    let cookie_store = CookieStore::default();
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_jar = Arc::new(cookie_store);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(Session::new(Arc::clone(&cookie_jar)))
        .invoke_handler(tauri::generate_handler![credit, logout, login, worker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
