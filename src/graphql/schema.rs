use std::{sync::Arc};
use async_graphql::{Object, Context, SimpleObject, InputObject};

use crate::state::AppState;
use crate::services::{get_aggregated_stats, ForgeRequest};

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

        let requests: Vec<ForgeRequest> = forges.into_iter().map(|f| ForgeRequest {
            name: f.name,
            username: f.username,
            token: f.token,
            url: f.url,
        }).collect();

        let raw_history = get_aggregated_stats(&state.providers, requests).await;

        let history = raw_history.into_iter().map(|(date, count)| DailyContribution {
            date,
            contribution_count: count
        }).collect();

        Some(Stats { history })
    }
}

