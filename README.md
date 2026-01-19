# ghstats-tool

a Rust based command-line tool that fetches and displays real time GitHub repository statistics using the GitHub REST API.

## Features

- Shows stars, forks, watchers, and open issues
- Displays last updated date
- Breaks down repository languages by percentage
- Supports GitHub authentication via personal access token

---

## Project Structure

```
ghstats-tool/
├── Cargo.toml
└── src/
    ├── main.rs      # entry point
    ├── cli.rs       # handles command line arguments
    ├── github.rs    # github api logic
    └── models.rs    # data structures to deserialize github api responses
```

---

## How it Works

1. The user runs the command with a repository owner and name: ``` ghstats-tool rust-lang rust ```
2. The CLI arguments are parsed in cli.rs and passed to main.rs.
3. main.rs calls async functions in github.rs to:
      - fetch repo metadata
      - fetch lang usage data
4. github.rs sends HTTP requests to the GitHub REST API:
      - adds a required User-Agent header
5. API responses are deserialized into Rust structs defined in models.rs.
6. The data is processed and formatted:
      - repo stats are printed directly
      - lang byte counts are converted into percentages
7. Results are displayed

---

## Usage

``` cargo run -- <owner> <repo> ```

