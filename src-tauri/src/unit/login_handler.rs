use std::time::Duration;

use super::Session;
use super::constants::{LOGIN_URL, CHANGEROLE_URL};

use serde_json::json;
use tauri::utils::html::parse;

pub async fn login_unit(
    username: &str,
    password: &str,
    state: &tauri::State<'_, Session>,
) -> Result<i32, i32> {
    state.clear_cookies();

    let payload = json!({
        "u": username,
        "p": password,
    });

    match state.client.post(LOGIN_URL).form(&payload).timeout(Duration::new(5, 0)).send().await {
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

                        match state.client.get(CHANGEROLE_URL).timeout(Duration::new(5, 0)).send().await {
                            Ok(crres) => {
                                if crres.url().path().contains("Welcome") {
                                    Ok(0)
                                } else {
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
pub async fn login(
    username: &str,
    password: &str,
    state: tauri::State<'_, Session>,
) -> Result<i32, i32> {
    let state = &state;
    loop {
        match login_unit(username, password, state).await {
            Ok(val) => {
                return Ok(val);
            }
            Err(1) => {
                return Err(0); // Err: wrong uname/pword
            }
            Err(_) => (),
        }
    }
}

#[tauri::command]
pub fn logout(state: tauri::State<Session>) {
    state.clear_cookies();
}
