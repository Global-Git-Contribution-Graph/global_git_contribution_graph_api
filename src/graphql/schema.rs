use std::{sync::Arc};
use async_graphql::{Object, Context, SimpleObject, InputObject};

use crate::state::AppState;
use crate::services::{get_aggregated_stats, transform_to_heatmap, ForgeRequest};

#[derive(SimpleObject, Debug, Clone)]
pub struct HeatmapCell {
    pub date: String,
    pub count: i64,
    pub level: i64 // 0 to 4
}

#[derive(SimpleObject, Debug, Clone)]
pub struct HeatmapWeek {
    pub days: Vec<HeatmapCell>
}

#[derive(SimpleObject)]
struct Stats {
    heatmap: Vec<HeatmapWeek>
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
    async fn stats(&self, ctx: &Context<'_>, uid: String, forges: Vec<ForgeInput>) -> Option<Stats> {
        let state = ctx.data::<Arc<AppState>>().ok()?;

        let requests: Vec<ForgeRequest> = forges.into_iter().map(|f| ForgeRequest {
            name: f.name,
            username: f.username,
            token: f.token,
            url: f.url,
        }).collect();

        let totals = get_aggregated_stats(&state.providers, &state.redis_client, uid, requests).await;

        let heatmap = transform_to_heatmap(totals);

        Some(Stats { 
            heatmap
        })
    }
}

