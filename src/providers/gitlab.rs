use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use chrono::{Local, Months, DateTime, Utc};

use crate::providers::{GitProvider};

pub struct GitLab;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GitLabPushData {
    commit_count: i64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GitLabEvent {
    created_at: DateTime<Utc>,
	push_data: Option<GitLabPushData>,
}

#[async_trait]
impl GitProvider for GitLab {
    fn get_name(&self) -> String {
        "GitLab".to_string()
    }

    async fn get_stats(&self, username: &str, token: &str, url: Option<&str>) -> Result<Vec<(String, i64)>, String> {
        let now = Local::now();
        let three_years_ago = now
        .checked_sub_months(Months::new(3 * 12))
        .expect("Date invalide");
        let after = three_years_ago.format("%Y-%m-%d").to_string();

        let per_page = 100;
        let mut page = 1;
        
        let client = reqwest::Client::new();
        let mut daily_counts: HashMap<String, i64> = HashMap::new();

        let url = url.ok_or("URL is required")?;

        loop {
            let complete_url = format!(
                "{}/api/v4/users/{}/events?action=pushed&after={}&per_page={}&page={}",
                url,
                username,
                after,
                per_page,
                page
            );

            println!("Called url : {}", complete_url);
            println!("Used token : {}", token);

            let res = client.get(complete_url)
                .header("PRIVATE-TOKEN", token)
                .header("User-Agent", "GGCG-App")
                .send()
                .await
                .map_err(|e| e.to_string())?;

            let events: Vec<GitLabEvent> = res.json::<Vec<GitLabEvent>>()
                .await
                .map_err(|e| e.to_string())?;
            let events_len = events.len();

            for event in events {
                let date = event.created_at.format("%Y-%m-%d").to_string();
                let count = event.push_data.map(|d| d.commit_count).unwrap_or(1);
                
                *daily_counts.entry(date).or_insert(0) += count;
            }

            if events_len < per_page {
                break;
            }
            page += 1;
        }

        let mut result: Vec<(String, i64)> = daily_counts.into_iter().collect();
        result.sort_by(|a, b| b.0.cmp(&a.0));

        Ok(result)
    }
}