# rust-task-tracker_cli

A small Rust command-line task tracker for managing TODOs and simple tasks locally. Designed to be minimal, fast, and scriptable.

## Features
- Add, list, complete, edit, and remove tasks
- Persistent local storage (JSON)
- Search and filter by status or text
- Export/import tasks
- Simple, human-readable output for piping and scripting

## Requirements
- Rust toolchain (rustc + cargo) — https://rustup.rs

## Installation
Clone and build locally:
```bash
git clone https://github.com/nXhermane/rust_task-tracker_cli.git
cd rust_task-tracker_cli
cargo build --release
# optional: install to cargo bin directory
cargo install --path .
```

## Quick start
Add a task:
```bash
rust_task-tracker_cli add "Buy groceries"
```

List tasks:
```bash
rust_task-tracker_cli list
```

Mark a task done:
```bash
rust_task-tracker_cli done 3
```

Remove a task:
```bash
rust_task-tracker_cli remove 4
```

Edit a task:
```bash
rust_task-tracker_cli edit 2 "Read Rust book chapter 7"
```

Export tasks:
```bash
rust_task-tracker_cli export tasks.json
```

Import tasks:
```bash
rust_task-tracker_cli import tasks.json
```

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
Dual-licensed under MIT OR Apache-2.0. See LICENSE file for details.

## Roadmap 
- Add priority and due-date metadata
- Subcommands for recurring tasks
- Sync backend (optional)

Questions or feature requests: open an issue in the repository.