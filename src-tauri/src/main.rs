#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use const_format::formatcp;
use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde_json::json;
use tauri::{utils::html::parse, Manager};
// use std::time::Duration;

const BASE_URL: &str = "https://academic.ui.ac.id/main/";
const LOGIN_URL: &str = formatcp!("{}Authentication/Index", BASE_URL);
const CHANGEROLE_URL: &str = formatcp!("{}Authentication/ChangeRole", BASE_URL);
// const SUMMARY_URL: &str = formatcp!("{}Authentication/Summary", BASE_URL);

struct Session {
    client: Client,
    cookies: Arc<CookieStoreMutex>,
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
    state.cookies.lock().unwrap().clear();

    let payload = json!({
        "u": username,
        "p": password,
    });

    match state.client.post(LOGIN_URL).form(&payload).send().await {
        Ok(res) => {
            println!("res: {:#?}", res);
            if res.status().is_success() {
                let content = res.text().await;
                match content {
                    Ok(text) => Ok(text),
                    Err(_) => Err("Empty".to_string()),
                }
            } else {
                Err("Error".to_string())
            }
        }
        Err(_) => Err("Error".to_string()),
    }
}
// TODO: deal with lifetime bullshits when divide the login subroutine chain
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
        .manage(Session {
            client: Client::builder()
                .cookie_store(true)
                .cookie_provider(Arc::clone(&cookie_jar))
                .build()
                .unwrap(),
            cookies: Arc::clone(&cookie_jar),
        })
        .invoke_handler(tauri::generate_handler![credit, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
