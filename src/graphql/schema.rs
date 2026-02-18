use std::{collections::HashMap, sync::Arc};
use async_graphql::{Object, Context, SimpleObject, InputObject};

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
        let mut totals: HashMap<String, i64> = HashMap::new();

        for forge in forges {
            let provider = state.providers.iter()
                .find(|p| p.get_name().to_lowercase() == forge.name.to_lowercase())?;

            let raw_stats = match provider.get_stats(&forge.username, &forge.token, forge.url.as_deref()).await {
                Ok(data) => data,
                Err(e) => {
                    println!("Erreur lors de l'appel API : {}", e);
                    return None;
                }
            };

            for (date, count) in raw_stats {
                *totals.entry(date).or_insert(0) += count;
            }
        }
        

        let mut history: Vec<DailyContribution> = totals.into_iter()
            .map(|(date, count)| DailyContribution { 
                date, 
                contribution_count: count
            })
            .collect();
        history.sort_by(|a, b| a.date.cmp(&b.date));

        Some(Stats { history })
    }
}

