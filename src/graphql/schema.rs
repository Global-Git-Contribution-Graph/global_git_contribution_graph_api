use std::sync::Arc;
use async_graphql::{Object, Context, SimpleObject};

use crate::state::AppState;

#[derive(SimpleObject)]
struct DailyContribution {
    date: String,
    contribution_count: i64
}

#[derive(SimpleObject)]
struct Stats {
    history: Vec<DailyContribution>
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn stats(&self, ctx: &Context<'_>, name: String, username: String, token: String) -> Option<Stats> {
        let state = ctx.data::<Arc<AppState>>().ok()?;

        let provider = state.providers.iter()
            .find(|p| p.get_name().to_lowercase() == name.to_lowercase())?;

        let raw_stats = match provider.get_stats(&username, &token).await {
            Ok(data) => data,
            Err(e) => {
                println!("Erreur lors de l'appel GitHub : {}", e);
                return None;
            }
        };

        let history = raw_stats.into_iter()
            .map(|(date, count)| DailyContribution { 
                date, 
                contribution_count: count 
            })
            .collect();

        Some(Stats { history })
    }
}

