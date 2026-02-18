use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use chrono::{DateTime};

use crate::providers::{GitProvider};

pub struct ForgeJo;

#[derive(Serialize, Deserialize, Debug)]
struct ForgejoContributions {
	timestamp: i64,
	contributions: i64
}

#[async_trait]
impl GitProvider for ForgeJo {
    fn get_name(&self) -> String {
        "ForgeJo".to_string()
    }

    async fn get_stats(&self, username: &str, token: &str, url: Option<&str>) -> Result<Vec<(String, i64)>, String> {
        let client = reqwest::Client::new();
        let mut daily_counts: HashMap<String, i64> = HashMap::new();

        let url = url.ok_or("URL is required")?;

        let complete_url = format!(
                "{}/api/v1/users/{}/heatmap?access_token={}",
                url,
                username,
                token
            );

        let res = client.get(complete_url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let contributions: Vec<ForgejoContributions> = res.json::<Vec<ForgejoContributions>>()
            .await
            .map_err(|e| e.to_string())?;

        for contribution in &contributions {
            let dt = DateTime::from_timestamp(contribution.timestamp, 0)
                .ok_or("invalid timestamp")?;
            let date_str = dt.format("%Y-%m-%d").to_string();

            *daily_counts.entry(date_str).or_insert(0) += contribution.contributions;
        }

        let mut formatted: Vec<(String, i64)> = Vec::new();
        for (date_str, total_count) in daily_counts {
            formatted.push((date_str, total_count));
        }

        Ok(formatted)
    }
}