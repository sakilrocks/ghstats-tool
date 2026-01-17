
use reqwest::Client;
use std::env;

use crate::models::{Languages, RepoInfo};



fn github_client() -> Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        "ghstats-tool".parse().unwrap(),
    );

    if let Ok(token) = env::var("GITHUB_TOKEN") {
        let value = format!("Bearer {}", token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            value.parse().unwrap(),
        );
    }

    Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build HTTP client")
}

pub async fn fetch_repo(owner: &str, repo: &str) -> Result<RepoInfo, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    let client = github_client();

    client.get(url).send().await?.json::<RepoInfo>().await
}

pub async fn fetch_languages(
    owner: &str,
    repo: &str,
) -> Result<Languages, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/languages",
        owner, repo
    );
    let client = github_client();

    client.get(url).send().await?.json::<Languages>().await
    
}