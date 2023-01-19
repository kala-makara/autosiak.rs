#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use const_format::formatcp;
use reqwest::blocking::Client;
use serde_json::json;
use tauri::Manager;
use tauri::utils::html::parse;
use tauri::http::header::CONTENT_TYPE;

const BASE_URL: &str = "https://academic.ui.ac.id/main/";
const LOGIN_URL: &str = formatcp!("{}Authentication/Index", BASE_URL);
// const CHANGEROLE_URL: &str = formatcp!("{}Authentication/ChangeRole", BASE_URL);
// const SUMMARY_URL: &str = formatcp!("{}Authentication/Summary", BASE_URL);

struct Session(Client);

#[tauri::command]
fn credit() {
    open::that("https://github.com/deadManAlive").unwrap_or(());
}

#[tauri::command]
fn login(username: &str, password: &str, client: tauri::State<Session>) -> String {
    let payload = json!({
        "u": username,
        "p": password,
    });

    let request = client.0
        .post(LOGIN_URL)
        .form(&payload)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded");

    println!("Acessing {} with {:?}", LOGIN_URL, request);

    let result = request.send();

    match result {
        Ok(res) => {
            match res.text() {
                Ok(content) => {
                    let html = parse(content);
                    match html.select_first("p.error") {
                        Ok(error_node_reference) => error_node_reference.text_contents(),
                        Err(_) => username.to_string()
                    }
                },
                Err(_) => String::from("Empty"),
            }
        },
        Err(_) => String::from("Error"),
    }
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
        #[cfg(debug_assertions)] {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
        }
        Ok(())
    })
    .manage(Session(Client::new()))
    .invoke_handler(tauri::generate_handler![credit, login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
