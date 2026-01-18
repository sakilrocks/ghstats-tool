
mod cli;
mod github;
mod models;

use clap::Parser;
use cli::Args;
use github::{fetch_languages, fetch_repo};




#[tokio::main]
async fn main() {
    let args = Args::parse();


    match fetch_repo(&args.owner, &args.repo).await {
        Ok(repo) => {
            println!("Repository: {}/{}", args.owner, args.repo);
            println!("Stars: {}", repo.stargazers_count);
            println!("Forks: {}", repo.forks_count);
            println!("Watchers: {}", repo.watchers_count);
            println!("Open issues: {}", repo.open_issues_count);
            println!("Last updated: {}", repo.updated_at);
        }
        Err(e) => {
            eprintln!("Failed to fetch repo info: {}", e);
            return;
        }
    }

    
    match fetch_languages(&args.owner, &args.repo).await {
        Ok(langs) => {
            println!("\nLanguages:");
            let total: u64 = langs.values().sum();

            let mut items: Vec<_> = langs.into_iter().collect();
            items.sort_by(|a, b| b.1.cmp(&a.1));

            for (lang, bytes) in items {
                let percent = (bytes as f64 / total as f64) * 100.0;
                println!("{}: {:.1}%", lang, percent);
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch languages: {}", e);
        }
    }

}