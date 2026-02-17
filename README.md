# tmx - Tmux Sessionizer

A fast and simple tmux session manager written in Rust.

## Features

- Detects if running inside or outside a tmux session
- Lists active tmux sessions with visual prefix
- Scans directories for project folders
- Creates new tmux sessions or attaches to existing ones
- Interactive directory/session selection

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/minigrep`.

## Usage

Simply run the binary:

```bash
minigrep
```

The tool will:
1. Detect if you're inside or outside a tmux session
2. Present an interactive list of active sessions and directories
3. Let you select a session to attach to or a directory to create a new session from

### When outside tmux:
- Select an active session (prefixed with `[TMUX]`) to attach to it
- Select a directory to create a new session and attach to it

### When inside tmux:
- Select an active session to switch to it
- Select a directory to create a new session and switch to it

## Configuration

Config file location: `~/.config/tmx/tmx.conf`

## Session Naming

Sessions are named using the last two components of the path. For example:
- `/home/user/projects/rust` becomes `projects/rust`
- `/home/user/go` becomes `user/go`

## Dependencies

- [walkdir](https://crates.io/crates/walkdir) - Directory traversal
- [inquire](https://crates.io/crates/inquire) - Interactive prompts
- [fuzzy-matcher](https://crates.io/crates/fuzzy-matcher) - Fuzzy matching

## Requirements

- tmux must be installed on your system

## License

MIT
