
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    pub owner: String,
    pub repo: String,
}