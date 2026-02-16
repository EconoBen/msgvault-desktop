# msgvault-desktop

Native desktop application for [msgvault](https://github.com/wesm/msgvault) email archive.

Built with **Rust** and the **Iced** GUI framework.

## Status

🚧 **Work in Progress** - Phase 1: Project Scaffolding

See [the plan](https://github.com/wesm/msgvault/pull/146) for the full roadmap.

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- A running `msgvault serve` instance

## Development

```bash
# Build
cargo build

# Run
cargo run

# Run with release optimizations
cargo run --release

# Run tests
cargo test
```

## Configuration

On first run, enter your msgvault server URL (e.g., `http://localhost:8080`) and optional API key.

Settings are saved to:
- macOS: `~/Library/Application Support/com.msgvault.msgvault-desktop/config.toml`
- Linux: `~/.config/msgvault-desktop/config.toml`
- Windows: `C:\Users\<User>\AppData\Roaming\msgvault\msgvault-desktop\config.toml`

## Architecture

```
┌─────────────────────────────┐     HTTP/REST     ┌─────────────────────────────┐
│  msgvault-desktop (Rust)    │  <───────────>    │  msgvault server (Go)       │
│  - Iced GUI framework       │                   │  - All existing endpoints   │
│  - reqwest HTTP client      │                   │  - SQLite/Parquet storage   │
└─────────────────────────────┘                   └─────────────────────────────┘
```

The desktop app is a thin HTTP client that talks to the msgvault server. All business logic remains in the Go server.

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # Iced Application implementation
├── message.rs       # Message enum (all events)
├── error.rs         # Error types
├── model/           # State (Model in MVU)
├── update/          # Message handlers (Update in MVU)
├── view/            # UI rendering (View in MVU)
│   └── widgets/     # Reusable components
├── api/             # HTTP client
│   ├── client.rs    # API client
│   └── types.rs     # Response types
└── config/          # Settings persistence
```

## Keyboard Shortcuts

Coming in Phase 2+. Will match the TUI keybindings:
- `j/k` or `↑/↓` - Navigate
- `Enter` - Drill down
- `Esc` - Go back
- `Tab` - Cycle views
- `/` - Search
- `?` - Help

## License

MIT
