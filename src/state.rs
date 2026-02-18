use std::sync::Arc;

use crate::providers::GitProvider;

pub struct AppState {
    pub providers: Vec<Arc<dyn GitProvider + Send + Sync>>,
}