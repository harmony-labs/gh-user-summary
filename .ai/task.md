HERE IS YOUR TASK:

Observe the example output - I am asking for events from March, but getting latest events, which happen to be in April.

This needs to be fixed.

LOG_LEVEL=debug cargo run -- patrickleet 2025-03
   Compiling gh-user-summary v0.1.0 (/Users/patrickleet/dev/patrickleet/gh-user-summary)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/gh-user-summary patrickleet 2025-03`
       Debug Starting gh-user-summary...
       Debug Command line args: ["target/debug/gh-user-summary", "patrickleet", "2025-03"]
        Info Processing for username: patrickleet, month: 2025-03
       Debug GITHUB_TOKEN found (length: 40)
       Debug Parsing date range for 2025-03
       Debug Naive date parsed: 2025-03-01
        Info Target range - Start: 2025-03-01 00:00:00 UTC, End: 2025-03-31 23:59:59 UTC
       Debug API URL: https://api.github.com/users/patrickleet/events?per_page=100
       Debug Adding Authorization header with token
       Debug Sending API request...
       Debug starting new connection: https://api.github.com/
       Debug Response status: 200 OK
        Info Total events received: 100
        Info Filtering events for range 2025-03-01 00:00:00 UTC to 2025-03-31 23:59:59 UTC
       Debug Event - Time: 2025-04-02 03:17:56 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 03:17:42 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 03:17:33 +00:00, Type: DeleteEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:56:51 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:54:50 +00:00, Type: DeleteEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:52:48 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:50:45 +00:00, Type: DeleteEvent, Repo: harmony-labs/workflow-vnext-tag, In range: false
       Debug Event - Time: 2025-04-02 02:49:49 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-vnext-tag, In range: false
       Debug Event - Time: 2025-04-02 02:48:33 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-vnext-tag, In range: false
       Debug Event - Time: 2025-04-02 02:48:19 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-vnext-tag, In range: false
       Debug Event - Time: 2025-04-02 02:47:06 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:46:12 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust-quality, In range: false
       Debug Event - Time: 2025-04-02 02:45:58 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-quality, In range: false
       Debug Event - Time: 2025-04-02 02:45:32 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:42:51 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-vnext-tag, In range: false
       Debug Event - Time: 2025-04-02 02:42:36 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-vnext-tag, In range: false
       Debug Event - Time: 2025-04-02 02:38:45 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:37:32 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-quality, In range: false
       Debug Event - Time: 2025-04-02 02:35:13 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust-quality, In range: false
       Debug Event - Time: 2025-04-02 02:29:29 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust-release, In range: false
       Debug Event - Time: 2025-04-02 02:18:00 +00:00, Type: CreateEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 02:16:56 +00:00, Type: PushEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 02:15:30 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 02:15:13 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 02:11:49 +00:00, Type: CreateEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 02:10:49 +00:00, Type: PushEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 02:05:04 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-02 02:03:52 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-02 02:03:35 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-02 02:01:44 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 02:01:23 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 01:59:31 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-02 01:56:29 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-02 01:55:49 +00:00, Type: PushEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-02 01:53:07 +00:00, Type: PushEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 01:53:07 +00:00, Type: PullRequestEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 01:52:27 +00:00, Type: PushEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 01:47:41 +00:00, Type: PushEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 01:44:33 +00:00, Type: PullRequestEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 01:44:24 +00:00, Type: PushEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 01:43:56 +00:00, Type: CreateEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 01:40:20 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 01:39:30 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 01:38:40 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 01:26:33 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 01:17:32 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 01:13:19 +00:00, Type: PushEvent, Repo: harmony-labs/harmony, In range: false
       Debug Event - Time: 2025-04-02 01:13:09 +00:00, Type: PushEvent, Repo: harmony-labs/harmony, In range: false
       Debug Event - Time: 2025-04-02 01:12:44 +00:00, Type: PushEvent, Repo: harmony-labs/harmony, In range: false
       Debug Event - Time: 2025-04-02 01:04:56 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-rust, In range: false
       Debug Event - Time: 2025-04-02 01:03:33 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-02 01:03:32 +00:00, Type: PullRequestEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-02 01:01:43 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-02 01:01:43 +00:00, Type: PullRequestEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-02 00:28:19 +00:00, Type: CreateEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 00:27:31 +00:00, Type: PushEvent, Repo: harmony-labs/contree-cli, In range: false
       Debug Event - Time: 2025-04-02 00:23:35 +00:00, Type: PushEvent, Repo: harmony-labs/harmony, In range: false
       Debug Event - Time: 2025-04-02 00:17:55 +00:00, Type: PushEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 23:37:57 +00:00, Type: CreateEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 23:37:11 +00:00, Type: PushEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 23:29:17 +00:00, Type: CreateEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 23:28:35 +00:00, Type: PushEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 23:24:33 +00:00, Type: CreateEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 23:23:46 +00:00, Type: PushEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 23:22:41 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 23:21:04 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 23:20:45 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 23:20:44 +00:00, Type: PullRequestEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 23:15:31 +00:00, Type: PushEvent, Repo: harmony-labs/action-vnext, In range: false
       Debug Event - Time: 2025-04-01 23:08:55 +00:00, Type: CreateEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 23:08:12 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 23:02:35 +00:00, Type: CreateEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 23:02:21 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 23:01:11 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 22:59:44 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:59:29 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:53:03 +00:00, Type: DeleteEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 22:51:52 +00:00, Type: PushEvent, Repo: harmony-labs/vnext, In range: false
       Debug Event - Time: 2025-04-01 22:49:31 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:46:21 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:46:04 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:45:33 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:45:18 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:44:16 +00:00, Type: CreateEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:44:03 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:40:18 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 22:39:27 +00:00, Type: PushEvent, Repo: harmony-labs/harmony, In range: false
       Debug Event - Time: 2025-04-01 22:37:21 +00:00, Type: PushEvent, Repo: harmony-labs/harmony, In range: false
       Debug Event - Time: 2025-04-01 22:14:43 +00:00, Type: PullRequestEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 22:14:36 +00:00, Type: PullRequestEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 22:14:30 +00:00, Type: CreateEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 21:39:29 +00:00, Type: PullRequestEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 21:39:19 +00:00, Type: CreateEvent, Repo: harmony-labs/gh-config-cli, In range: false
       Debug Event - Time: 2025-04-01 20:57:19 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 20:57:18 +00:00, Type: PullRequestEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 20:55:11 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 20:51:57 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 20:50:02 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 20:31:13 +00:00, Type: PushEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Event - Time: 2025-04-01 20:31:12 +00:00, Type: PullRequestEvent, Repo: harmony-labs/workflow-release, In range: false
       Debug Daily summaries after filtering: {}
        Warn No events found for patrickleet in the specified range.