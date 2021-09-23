use crate::services::user::{SessionStatusCallback, UserSession, UserSessionConfig};
use std::sync::Arc;

pub struct UserSessionBuilder {
    config: Option<UserSessionConfig>,
    callback: SessionStatusCallback,
}

impl UserSessionBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            callback: Arc::new(|_| {}),
        }
    }

    pub fn root_dir(mut self, dir: &str) -> Self {
        self.config = Some(UserSessionConfig::new(dir));
        self
    }

    pub fn status_callback(mut self, callback: SessionStatusCallback) -> Self {
        self.callback = callback;
        self
    }

    pub fn build(mut self) -> UserSession {
        let config = self.config.take().unwrap();
        UserSession::new(config, self.callback.clone())
    }
}