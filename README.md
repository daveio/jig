# hubbit

git checkout/clone manager and binary downloader

## Config

Default config path: `~/.config/ghc/config.toml`

### Example configuration file

```toml
[main]
# Use libgit to perform git operations if true.
# Use the git CLI to perform git operations if false.
libgit = false # true not yet implemented

[github]
# Your GitHub username.
username = abc123
# Your GitHub token.
token = ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx # Try `gh auth token`.
# Use ssh to clone GitHub repositories that you own.
use_ssh = true
```

## Invocations

### `ghc clone repo`

Clone `github.com/YOURUSERNAME/repo`.

Uses SSH if `github.use_ssh` is `true` in the configuration file, HTTPS
otherwise.

Your username must be set in the configuration file.

### `ghc clone username/repo`

Clone `github.com/username/repo` using HTTPS.

### `ghc clone hostname.com/repo`

Clone `hostname.com/repo` using HTTPS.

### `ghc clone https://hostname.com/repo`

Clone `hostname.com/repo` using HTTPS.

### `ghc clone hostname.com:/repo`

Clone `hostname.com:/repo` using SSH. Uses your username.

### `ghc clone git@hostname.com:/repo`

Clone `hostname.com/repo` using SSH. Uses the specified username.

## Flow

```mermaid
graph TD
    A(Invoke) --> B(Parse opts with clap)
    B --> D(Forward slash?)

    D -- No forward slash --> E("reponame"):::mono
    D -- Forward slash --> F("username/reponame"):::mono
    F --> O

    E --> K("github.username"):::mono
    K -- Present --> O("Clone with git CLI")
    K -- Missing --> L("Abort")

    subgraph Config
        K
    end

    classDef mono font-family: monospace;
```

### Pattern Matching

Since we're only dealing with GitHub repos, the possible inputs are

- `REPONAME`
- `USERNAME/REPONAME`

All we need to do to distinguish the two is to search for a `/` which cannot be present in usernames or repository names asdf asf asf
