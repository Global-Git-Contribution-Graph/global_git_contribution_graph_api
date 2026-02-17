use std::sync::Arc;
use async_graphql::{Object, Context, SimpleObject};

use crate::state::AppState;

#[derive(SimpleObject)]
struct Stats {
    contributions: i32,
    username: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn stats(&self, ctx: &Context<'_>, name: String) -> Option<Stats> {
        let state = ctx.data::<Arc<AppState>>().ok()?;

        let provider = state.providers.iter()
            .find(|p| p.get_name().to_lowercase() == name.to_lowercase())?;

        Some(Stats {
            contributions: 42,
            username: format!("User sur {}", provider.get_name()),
        })
    }
}

