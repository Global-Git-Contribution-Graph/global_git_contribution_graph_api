use std::collections::HashMap;
use std::sync::Arc;
use futures::future::join_all;

use crate::providers::GitProvider;

pub struct ForgeRequest {
    pub name: String,
    pub username: String,
    pub token: String,
    pub url: Option<String>,
}

pub async fn get_aggregated_stats(providers: &[Arc<dyn GitProvider + Send + Sync>], forges: Vec<ForgeRequest>) -> Vec<(String, i64)> {
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

    let mut history: Vec<(String, i64)> = totals
        .into_iter()
        .collect();
    history.sort_by(|a, b| a.0.cmp(&b.0));

    history
}