use scraper::{Html, Selector};
use std::{collections::HashMap, time::Duration};

use super::{
    constants::FEError::*,
    constants::{FEError, NullFEResult, CHANGEROLE_URL, LOGIN_URL},
    Session,
};

async fn login_unit(
    username: &str,
    password: &str,
    state: &tauri::State<'_, Session>,
) -> NullFEResult {
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
        .await
        .map_err(|_| LoginInaccessible)
        .map(|x| async {
            let content = x.text().await.map_err(|_| LoginLoadError)?;

            let document = Html::parse_document(&content);
            let meta_selector = Selector::parse("meta[http-equiv='Refresh']").unwrap();

            let refresh_meta = document.select(&meta_selector);
            Ok::<usize, FEError>(refresh_meta.count())
        })?
        .await?;

    if login_success < 1 {
        return Err(LoginFailed);
    }

    Ok(())
}

async fn change_role_unit(state: &tauri::State<'_, Session>) -> NullFEResult {
    let role_changed = state
        .client
        .get(CHANGEROLE_URL)
        .timeout(Duration::new(5, 0))
        .send()
        .await
        .map_err(|_| ChangeRoleFailed)?;

    let redirected = role_changed.url().path() == "/main/Welcome/";

    if redirected {
        Ok(())
    } else {
        Err(MainPageInaccessible)
    }
}

#[tauri::command]
pub async fn login(
    username: &str,
    password: &str,
    state: tauri::State<'_, Session>,
) -> NullFEResult {
    login_unit(username, password, &state)
        .await
        .map(|_| async { change_role_unit(&state).await })?
        .await?;

    Ok(())
}

#[tauri::command]
pub fn logout(state: tauri::State<Session>) {
    state.clear_cookies();
}
