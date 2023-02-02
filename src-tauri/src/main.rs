#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use const_format::formatcp;
use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde_json::json;
use tauri::utils::html::parse;
use tauri::Manager;

const BASE_URL: &str = "https://academic.ui.ac.id/main";
const LOGIN_URL: &str = formatcp!("{}/Authentication/Index", BASE_URL);
const CHANGEROLE_URL: &str = formatcp!("{}/Authentication/ChangeRole", BASE_URL);
// const SCHEDULE_URL: &str = formatcp!("{}/Schedule/Index", BASE_URL);
// const SUMMARY_URL: &str = formatcp!("{}Authentication/Summary", BASE_URL);

struct Session {
    client: Client,
    cookies: Arc<CookieStoreMutex>,
}

impl Session {
    fn new(cookie_store: Arc<CookieStoreMutex>) -> Session {
        Session {
            client: Client::builder()
                .cookie_store(true)
                .cookie_provider(Arc::clone(&cookie_store))
                .build()
                .unwrap(),
            cookies: Arc::clone(&cookie_store),
        }
    }

    fn clear_cookies(&self) {
        self.cookies.lock().unwrap().clear();
    }
}

#[tauri::command]
fn credit() {
    open::that("https://github.com/deadManAlive").unwrap_or(());
}

/// # Login Unit
#[tauri::command]
async fn login(
    username: &str,
    password: &str,
    state: tauri::State<'_, Session>,
) -> Result<String, i32> {
    state.clear_cookies();

    let payload = json!({
        "u": username,
        "p": password,
    });

    match state.client.post(LOGIN_URL).form(&payload).send().await {
        Ok(res) => {
            if res.status().is_success() {
                let content = res.text().await;
                match content {
                    Ok(text) => {
                        let is_error = {
                            let login_resp_page = parse(text);
                            login_resp_page.select_first("p.error").is_ok()
                        }; // TODO: RTFM async vs. threading

                        if is_error {
                            return Err(1); // Err 1: Wrong Creds
                        }

                        match state.client.get(CHANGEROLE_URL).send().await {
                            Ok(crres) => {
                                if crres.url().path().contains("Welcome") {
                                    Ok(username.to_string())
                                }
                                else {
                                    Err(2) // Err 2: ChangeRole request failure
                                }
                            }
                            Err(_) => {
                                Err(3) // Err 3: ChangeRole no response
                            }
                        }
                    }
                    Err(_) => Err(4), // Err 4: Login no response
                }
            } else {
                Err(5) // Err 5: login url fail to respond
            }
        }
        Err(_) => Err(6), // Err 6: fail to send request to login url
    }
}

#[tauri::command]
fn logout(state: tauri::State<Session>) {
    state.clear_cookies();
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
        .invoke_handler(tauri::generate_handler![credit, login, logout])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
