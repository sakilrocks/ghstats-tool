
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug, Deserialize)]
pub struct RepoInfo {
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    pub watchers_count: u64,
    pub updated_at: String,
}

pub type Languages = HashMap<String, u64>;