# GitHub Contributions Summary

A simple Rust command-line tool to fetch and summarize your GitHub contributions for a given month, displaying daily activity with start and end timestamps.

## Purpose
This tool retrieves your GitHub events for a specified month and presents a clean, human-readable summary of your daily contributions. It’s built with simplicity and clarity in mind, avoiding unnecessary complexity.

## Features
- Fetches public and private GitHub events (with authentication).
- Groups contributions by day.
- Shows start and end timestamps for each day’s activity.
- Lists event types and repository names.
- Includes detailed logging for debugging.

## Installation
1. **Clone the repository:**
   ```bash
   git clone https://github.com/patrickleet/gh-user-summary.git
   cd gh-user-summary