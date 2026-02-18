use serde::{Deserialize, Serialize};
use serde_json::json;
use async_trait::async_trait;

use crate::providers::{GitProvider};

pub struct GitHub;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContributionDay {
    date: String,
    contribution_count: i64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Week {
    contribution_days: Vec<ContributionDay>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContributionCalendar {
    weeks: Vec<Week>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContributionsCollection {
    contribution_calendar: ContributionCalendar
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    contributions_collection: ContributionsCollection
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    user: User
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GithubResponse {
    data: Data
}

#[async_trait]
impl GitProvider for GitHub {
    fn get_name(&self) -> String {
        "GitHub".to_string()
    }

    async fn get_stats(&self, username: &str, token: &str, _url: Option<&str>) -> Result<Vec<(String, i64)>, String> {
        let query = "
            query($login: String!) {
                user(login: $login) {
                    contributionsCollection {
                        contributionCalendar {
                            weeks {
                                contributionDays {
                                    date
                                    contributionCount
                                }
                            }
                        }
                    }
                }
            }";

        let body = json!({
            "query": query,
            "variables": {
                "login": username
            }
        });
        
        let client = reqwest::Client::new();
        let res = client.post("https://api.github.com/graphql".trim_end_matches('/'))
            .header("Authorization", format!("bearer {}", token))
            .header("User-Agent", "GGCG-App")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let github_resp: GithubResponse = res.json::<GithubResponse>()
            .await
            .map_err(|e| e.to_string())?;

        let mut formatted_contributions: Vec<(String, i64)> = Vec::new();
        for week in github_resp.data.user.contributions_collection.contribution_calendar.weeks {
            for day in week.contribution_days {
                if day.contribution_count == 0 {
                    continue;
                }

                formatted_contributions.push((day.date.to_string(), day.contribution_count));
            }
        }

        Ok(formatted_contributions)
    }
}