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
/// How this works:
/// 1. Clear cookies.
/// 2. POST form payload.
/// 3. Match response:
///     * success: parse for login err
///         * no login err:
///             1. get change_role
///             2. return `Ok(Some(username))`
///         * login err: return `Ok(None)` -> panic/stop
///     * failed: return `Err(())` -> sign for looping
#[tauri::command]
async fn login(
    username: &str,
    password: &str,
    state: tauri::State<'_, Session>,
) -> Result<String, String> {
    // state.cookies.lock().unwrap().clear();
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
                            // println!("error? {:#?}", login_resp_page.select_first("p.error"));
                            login_resp_page.select_first("p.error").is_ok()
                        }; // TODO: RTFM async vs. threading

                        if is_error {
                            return Ok("Failed".to_string());
                        }

                        match state.client.get(CHANGEROLE_URL).send().await {
                            Ok(crres) => {
                                if crres.url().path().contains("Welcome") {
                                    Ok(username.to_string())
                                }
                                else {
                                    Ok("Failed".to_string())
                                }
                            }
                            Err(err) => {
                                println!("{:?}", err);
                                Ok("Error".to_string())
                            }
                        }
                        // Ok(text)
                    }
                    Err(_) => Ok("Empty".to_string()),
                }
            } else {
                Ok("Error".to_string())
            }
        }
        Err(_) => Ok("Error".to_string()),
    }
}

#[tauri::command]
fn logout(state: tauri::State<Session>) {
    state.clear_cookies();
    println!("cookies: {:#?}", state.cookies);
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
