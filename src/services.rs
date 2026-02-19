use std::collections::HashMap;
use std::sync::Arc;
use futures::future::join_all;
use chrono::{Local, Duration, Datelike};
use redis::AsyncCommands;

use crate::providers::GitProvider;
use crate::graphql::schema::{HeatmapWeek, HeatmapCell};

pub struct ForgeRequest {
    pub name: String,
    pub username: String,
    pub token: String,
    pub url: Option<String>,
}

pub async fn get_aggregated_stats(providers: &[Arc<dyn GitProvider + Send + Sync>], redis_client: &redis::Client, uid: String, forges: Vec<ForgeRequest>) -> HashMap<String, i64> {
    let cache_key = format!("cache:{}", uid);
    
    if let Ok(mut conn) = redis_client.get_multiplexed_async_connection().await {
        let cached_data: Option<String> = conn.get(&cache_key).await.unwrap_or(None);

        if let Some(json_str) = cached_data {
            if let Ok(cached_totals) = serde_json::from_str::<HashMap<String, i64>>(&json_str) {
                return cached_totals;
            }
        }
    }

    // Cache miss
    let totals: HashMap<String, i64> = fetch_stats(providers, forges).await;
 
    if let Ok(mut conn) = redis_client.get_multiplexed_async_connection().await {
        if let Ok(json_str) = serde_json::to_string(&totals) {
            let _: redis::RedisResult<()> = conn.set_ex(&cache_key, json_str, 3600).await;
        }
    }

    totals
}

pub async fn fetch_stats(providers: &[Arc<dyn GitProvider + Send + Sync>], forges: Vec<ForgeRequest>) -> HashMap<String, i64> {
    let mut tasks = Vec::new();

    for forge in forges {
        if let Some(provider) = providers.iter().find(|p| p.get_name().eq_ignore_ascii_case(&forge.name)) {
            let provider = provider.clone();
            tasks.push(async move {
                provider.get_stats(&forge.username, &forge.token, forge.url.as_deref()).await
            });
        } else {
            eprintln!("Unknow provider : {}", forge.name);
        }
    }

    let results: Vec<Result<Vec<(String, i64)>, String>> = join_all(tasks).await;
    let mut totals: HashMap<String, i64> = HashMap::new();

    for result in results {
        match result {
            Ok(raw_stats) => {
                for (date, count) in raw_stats {
                    *totals.entry(date).or_insert(0) += count;
                }
            }
            Err(e) => {
                eprintln!("Error calling API: {}", e);
            }
        }
    }

    totals
}

pub fn transform_to_heatmap(totals: HashMap<String, i64>) -> Vec<HeatmapWeek> {
    let today = Local::now().date_naive();

    let one_year_ago = today - Duration::weeks(52);
    let days_from_sunday = one_year_ago.weekday().num_days_from_sunday();
    let start_date = one_year_ago - Duration::days(days_from_sunday as i64);

    let mut weeks: Vec<HeatmapWeek> = Vec::new();
    let mut current_week_days: Vec<HeatmapCell> = Vec::new();

    // max calculation for color scale
    let max_contribution = totals.values().max().cloned().unwrap_or(0);

    let mut current_date = start_date;

    while current_date <= today || current_date.weekday().num_days_from_sunday() != 0 {
        let date_str = current_date.format("%Y-%m-%d").to_string();
        let count = *totals.get(&date_str).unwrap_or(&0);

        let level = calculate_level(count, max_contribution);

        current_week_days.push(HeatmapCell {
            date: date_str,
            count,
            level,
        });

        // if we have seven days, we close for the week
        if current_week_days.len() == 7 {
            weeks.push(HeatmapWeek { days: current_week_days });
            current_week_days = Vec::new();
        }

        current_date += Duration::days(1);
        
        // security to prevent an infinite loop if the date logic fails
        if weeks.len() > 54 { break; } 
    }

    if !current_week_days.is_empty() {
        weeks.push(HeatmapWeek { days: current_week_days });
    }

    weeks
}

fn calculate_level(count: i64, max: i64) -> i64 {
    if count == 0 { return 0; }
    if max <= 0 { return 0; }

    let ratio = count as f64 / max as f64;
    
    if ratio <= 0.25 { 1 }
    else if ratio <= 0.50 { 2 }
    else if ratio <= 0.75 { 3 }
    else { 4 }
}