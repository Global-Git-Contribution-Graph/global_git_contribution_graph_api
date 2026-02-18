pub mod providers;
pub mod github;
pub mod gitlab;
pub mod forgejo;

pub use providers::GitProvider;
pub use github::GitHub;
pub use gitlab::GitLab;
pub use forgejo::ForgeJo;