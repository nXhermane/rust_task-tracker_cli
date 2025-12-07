# rust-task-tracker_cli

A small Rust command-line task tracker for managing TODOs and simple tasks locally. Designed to be minimal, fast, and scriptable.

## Features
- Add, list, complete, edit, and remove tasks
- Persistent local storage (JSON)
- Simple, human-readable output for piping and scripting

## Requirements
- Rust toolchain (rustc + cargo) — https://rustup.rs

## Installation
Clone and build locally:
```bash
git clone https://github.com/nXhermane/task-tracker-cli.git
cd task-tracker-cli
cargo build --release
# optional: install to cargo bin directory
cargo install --path .
```

## Quick start
Add a task:
```bash
task-tracker-cli add "Buy groceries"
```

List tasks:
```bash
task-tracker-cli list
```

Mark a task done:
```bash
task-tracker-cli done 3
```
Mark a task in progress: 
```bash 
task-tracker-cli start 3
```
Remove a task:
```bash
task-tracker-cli remove 4
```

Edit a task:
```bash
task-tracker-cli edit 2 "Read Rust book chapter 7"
```

<!-- Export tasks:
```bash
task-tracker-cli export tasks.json
```

Import tasks:
```bash
task-tracker-cli import tasks.json
``` -->

Run in development:
```bash
cargo run -- <command> [...]
```

## Storage
Tasks are stored in a simple local file (e.g. in the project directory or a user data directory). The format is human-readable (JSON) to allow manual edits and easy interoperability.

## Testing & Linting
Run tests:
```bash
cargo test
```

Format and lint:
```bash
cargo fmt
cargo clippy -- -D warnings
```

## Contributing
- Fork the repo, create a branch, and open a pull request.
- Follow idiomatic Rust and include tests for new behavior.
- Keep changes small and focused.

## License
Licenced MIT. See [LICENSE](./LICENSE) file for details.

## Roadmap 
- Search and filter by status or text
- Export/import tasks
- Add priority and due-date metadata
- Subcommands for recurring tasks
- Sync backend (optional)

Questions or feature requests: open an issue in the repository.