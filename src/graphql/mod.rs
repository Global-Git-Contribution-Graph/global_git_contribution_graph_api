pub mod schema;
pub mod handlers;

pub use schema::QueryRoot;
pub use handlers::{graphql_handler, graphiql};