use std::time::Duration;

use super::Session;
use super::constants::COURSEPLANEDIT_URL;

pub async fn scraper(state: &tauri::State<'_, Session>) -> Result<i32, i32> {
    match state.client.get(COURSEPLANEDIT_URL).timeout(Duration::new(5, 0)).send().await {
        Ok(res) => {
            if res.status().is_success() {
                println!("CPE: {:#?}", res); // NOTE: debug response of course plan edit
                Ok(0)
            } else {
                Err(0)
            }
        }
        Err(_) => Err(1),
    }
}

#[tauri::command]
pub async fn worker(state: tauri::State<'_, Session>) -> Result<String, i32> {
    let state = &state;
    scraper(state).await.unwrap();
    Err(0)
}