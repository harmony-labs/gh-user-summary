# gh-user-summary

A lightweight Rust CLI tool to summarize your GitHub contributions over a specified period.

## Overview

**gh-user-summary** fetches your GitHub events (up to 300 recent events) using the GitHub API and provides a daily breakdown of your contributions. It supports filtering by a custom date range, a specific month, or a single day. With a focus on simplicity and clarity, the tool leverages concurrency and disk caching for efficient processing.

## Features

- **Date-Based Filtering**: Summarize events for a custom date range, a particular month, or a specific day.
- **Detailed Summaries**: Get a breakdown of events including push events with commit messages, pull request events, and repository creation/deletion.
- **Concurrency**: Uses Rayon for parallel processing of events.
- **Disk Caching**: Caches API responses using `cacache` to reduce redundant network calls.
- **Configurable**: Easily configure via command-line arguments and environment variables.

## Installation

### Using ubi

1. **Install ubi:**  
   Ensure you have ubi installed by running:
   ```bash
   mkdir -p ~/.ubi/bin
   echo 'export PATH="$HOME/.ubi/bin:$PATH"' >> ~/.zshrc  # or your preferred shell profile
   ```
2. **Install vnext with ubi:**  
   ```bash
   ubi --project harmony-labs/vnext --in ~/.ubi/bin

## Usage

Run the binary with the following options:

```bash
gh-user-summary --username <GitHubUsername> [--start-date YYYY-MM-DD --end-date YYYY-MM-DD | --month YYYY-MM | --day YYYY-MM-DD]
```

### Examples

- **Summarize a Month**:

  ```bash
  gh-user-summary --username octocat --month 2023-05
  ```

- **Summarize a Specific Day**:

  ```bash
  gh-user-summary --username octocat --day 2023-05-15
  ```

- **Summarize a Custom Date Range**:

  ```bash
  gh-user-summary --username octocat --start-date 2023-05-01 --end-date 2023-05-31
  ```

## Configuration

### Environment Variables

- **GITHUB_TOKEN**: _(Optional)_ Your personal GitHub token for authenticated API requests.
- **LOG_LEVEL**: Set the desired log level (e.g., `info`, `debug`, `warn`, `error`). Defaults to `info` if not set.

## Project Structure

```
gh-user-summary/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── renovate.json
├── .github
│   └── workflows
│       ├── on-push-main-version-and-tag.yaml
│       ├── on-v-tag-release.yaml
│       └── on-pr-quality.yaml
├── .ai
│   └── context.md
└── src
    ├── main.rs          # Entry point: parses CLI args and orchestrates the flow
    ├── api.rs           # Handles API calls and caching logic
    ├── events.rs        # Processes and filters GitHub events
    ├── logging.rs       # Initializes logging with colored output
    └── summary.rs       # Formats and prints the summary output
```

## CI/CD and Automation

The repository is set up with GitHub Actions workflows to:

- **Ensure Quality**: Run quality checks on pull requests.
- **Automate Versioning & Releases**: Automatically version and tag releases on pushes to the main branch.
- **Build & Publish Binaries**: Build and publish Rust binaries when a new version tag is pushed.

## Contributing

Contributions are welcome! Please follow these guidelines:

- **Keep It Simple**: Propose minimal changes that fix the issue.
- **Stay Clear & Concise**: Match the existing coding style and structure.
- **Test & Log**: Include targeted logging and testing to ensure your changes work as expected.

For additional context on coding style and philosophy, refer to the `.ai/context.md` file.

## License

[Insert License Information Here]

## Acknowledgements

This project leverages a variety of excellent open-source libraries, including:
- [reqwest](https://github.com/seanmonstar/reqwest)
- [chrono](https://github.com/chronotope/chrono)
- [serde](https://serde.rs)
- [rayon](https://github.com/rayon-rs/rayon)
- [clap](https://github.com/clap-rs/clap)
- [fern](https://github.com/daboross/fern)
- [cacache](https://github.com/zkat/cacache)

Happy summarizing!

---

Let me know if you’d like to proceed with this README or if you need any adjustments.
