use reqwest::Error as ReqwestError;
use std::{collections::HashMap, time::Duration};
use scraper::{Html, Selector};

use super::{constants::LOGIN_URL, Session};

async fn login_unit(
    username: &str,
    password: &str,
    state: &tauri::State<'_, Session>,
) -> Result<(), ReqwestError> {
    state.clear_cookies();

    let mut payload = HashMap::new();
    payload.insert("u", username);
    payload.insert("p", password);

    let login_success = state
        .client
        .post(LOGIN_URL)
        .form(&payload)
        .timeout(Duration::new(5, 0))
        .send()
        .await.map(|x| async {
            let content = x.text().await.unwrap();
            let document = Html::parse_document(&content);
            let meta_selector = Selector::parse("meta[http-equiv='Refresh']").unwrap();
            let refresh_meta = document.select(&meta_selector);
            refresh_meta.count()
        })?.await;

    if login_success < 1 {
        state.client.get("[u/p]").send().await?;
    }

    Ok(())
}

// TODO: ChangeRole isn't acessed yet, separate unit?

#[tauri::command]
pub async fn login(
    username: &str,
    password: &str,
    state: tauri::State<'_, Session>,
) -> Result<(), String> {
    login_unit(username, password, &state)
        .await
        .map_err(|err| err.to_string() )?;

    Ok(())
}

#[tauri::command]
pub fn logout(state: tauri::State<Session>) {
    state.clear_cookies();
}
