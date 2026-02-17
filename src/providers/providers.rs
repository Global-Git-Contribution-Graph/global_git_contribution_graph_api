use async_trait::async_trait;

#[async_trait]
pub trait GitProvider {
    fn get_name(&self) -> String;
    async fn get_stats(&self, username: &str, token: &str) -> Result<Vec<(String, i64)>, String>;
}

