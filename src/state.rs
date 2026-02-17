use crate::providers::GitProvider;

pub struct AppState {
    pub providers: Vec<Box<dyn GitProvider + Send + Sync>>,
}