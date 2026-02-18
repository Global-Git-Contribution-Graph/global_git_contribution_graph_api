use std::{collections::HashMap, sync::Arc};
use async_graphql::{Object, Context, SimpleObject, InputObject};
use futures::future::join_all;

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

#[derive(InputObject)]
struct ForgeInput {
    name: String,
    username: String,
    token: String,
    url: Option<String>
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn stats(&self, ctx: &Context<'_>, forges: Vec<ForgeInput>) -> Option<Stats> {
        let state = ctx.data::<Arc<AppState>>().ok()?;
        let mut tasks = Vec::new();

        for forge in forges {
            let provider = state.providers
                .iter()
                .find(|p| p.get_name().to_lowercase() == forge.name.to_lowercase())?
                .clone();

            tasks.push(async move {
                provider.get_stats(&forge.username, &forge.token, forge.url.as_deref()).await
            });
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
                    println!("Erreur lors de l'appel API : {}", e);
                }
            }
        }

        let mut history: Vec<DailyContribution> = totals
            .into_iter()
            .map(|(date, contribution_count)| DailyContribution { date, contribution_count })
            .collect();
        history.sort_by(|a, b| a.date.cmp(&b.date));

        Some(Stats { history })
    }
}

