pub mod constants;

pub mod login_handler;
pub mod worker_handler;

use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;

pub struct Session {
    pub client: Client,
    pub cookies: Arc<CookieStoreMutex>,
}

impl Session {
    pub fn new(cookie_store: Arc<CookieStoreMutex>) -> Session {
        Session {
            client: Client::builder()
                .cookie_store(true)
                .cookie_provider(Arc::clone(&cookie_store))
                .build()
                .unwrap(),
            cookies: Arc::clone(&cookie_store),
        }
    }

    pub fn clear_cookies(&self) {
        self.cookies.lock().unwrap().clear();
    }
}
