#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use const_format::formatcp;
use reqwest::blocking::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde_json::json;
use tauri::{utils::html::parse, Manager};
use std::time::Duration;

const BASE_URL: &str = "https://academic.ui.ac.id/main/";
const LOGIN_URL: &str = formatcp!("{}Authentication/Index", BASE_URL);
const CHANGEROLE_URL: &str = formatcp!("{}Authentication/ChangeRole", BASE_URL);
// const SUMMARY_URL: &str = formatcp!("{}Authentication/Summary", BASE_URL);

struct Session(Client, Arc<CookieStoreMutex>);

#[tauri::command]
fn credit() {
    open::that("https://github.com/deadManAlive").unwrap_or(());
}

//TODO: clear cookie, or else this is useless
#[tauri::command]
fn login(username: &str, password: &str, client: tauri::State<Session>) -> String {
    let mut store = client.1.lock().unwrap();

    println!("cookie: {:#?}", store);

    let payload = json!({
        "u": username,
        "p": password,
    });

    store.clear();

    let request = client.0.post(LOGIN_URL).form(&payload).timeout(Duration::new(5, 0));

    println!("Acessing {} with {:#?}", LOGIN_URL, request);

    let result = request.send();

    println!("Responded with {:#?}", result);

    match result {
        Ok(res) => match res.text() {
            Ok(content) => {
                let html = parse(content);
                match html.select_first("p.error") {
                    Ok(error_node_reference) => error_node_reference.text_contents(),
                    Err(_) => {
                        let changerole = client.0.get(CHANGEROLE_URL).timeout(Duration::new(5, 0));

                        println!("Change role request: {:#?}", changerole);

                        let changerole_result = changerole.send();

                        if let Ok(resp) = changerole_result {
                            println!("change role response: {}", resp.text().unwrap());
                        }

                        username.to_string()
                    }
                }
            }
            Err(_) => String::from("Empty"),
        },
        Err(_) => String::from("Error"),
    }
}

// #[tauri::command]
// fn get_user_detail(query: &str, client: tauri::State<Session>) {
//     let request = client.0
//         .get()
// }

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
        .manage(Session(
            Client::builder()
                .cookie_store(true)
                .cookie_provider(Arc::clone(&cookie_jar))
                .build()
                .unwrap(),
            Arc::clone(&cookie_jar),
        ))
        .invoke_handler(tauri::generate_handler![credit, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
