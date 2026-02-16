# CLAUDE.md - Development Guide

## Project Overview

msgvault-desktop is a native desktop application for browsing email archives. It uses:
- **Rust** - Systems programming language
- **Iced** - GUI framework (Elm-inspired MVU architecture)
- **reqwest** - HTTP client for API calls
- **serde** - JSON serialization

The app talks to a `msgvault serve` backend via HTTP API.

## Quick Commands

```bash
# Build
cargo build

# Run (debug)
cargo run

# Run (release, optimized)
cargo run --release

# Test
cargo test

# Check (fast compile check without building)
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Architecture: MVU (Model-View-Update)

Iced uses the Elm architecture:

1. **Model** (`src/model/`) - Application state
2. **View** (`src/view/`) - Renders UI from state
3. **Update** (`src/update/`) - Handles messages, updates state
4. **Message** (`src/message.rs`) - All possible events

```rust
// Data flow:
User Action → Message → Update(state, message) → New State → View(state) → UI
```

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | Entry point, runs Iced |
| `src/app.rs` | Main Application struct |
| `src/message.rs` | All Message variants |
| `src/model/state.rs` | AppState struct |
| `src/update/mod.rs` | Message handlers |
| `src/view/mod.rs` | UI rendering |
| `src/api/client.rs` | HTTP client |
| `src/api/types.rs` | API response types |
| `src/config/mod.rs` | Settings persistence |

## Adding a New Feature

1. **Define Messages** in `src/message.rs`:
```rust
pub enum Message {
    // Add new variants
    FetchStats,
    StatsLoaded(Result<StatsResponse, AppError>),
}
```

2. **Update State** in `src/model/state.rs`:
```rust
pub struct AppState {
    // Add new fields
    pub stats: Option<StatsResponse>,
}
```

3. **Handle Messages** in `src/update/mod.rs`:
```rust
Message::FetchStats => {
    Task::perform(async { ... }, Message::StatsLoaded)
}
Message::StatsLoaded(result) => {
    state.stats = result.ok();
    Task::none()
}
```

4. **Render UI** in `src/view/mod.rs`:
```rust
if let Some(stats) = &state.stats {
    text(format!("Messages: {}", stats.total_messages))
}
```

## API Client Usage

```rust
use crate::api::ApiClient;

// Create client
let client = ApiClient::new("http://localhost:8080", Some("api-key".to_string()));

// Make request (async)
let health = client.health().await?;
```

## Iced Patterns

### Async Operations

```rust
// In update handler:
Task::perform(
    async { api_call().await },
    |result| Message::ResultReceived(result),
)
```

### Widget Composition

```rust
column![
    text("Title").size(24),
    Space::with_height(10),
    button("Click").on_press(Message::Clicked),
]
.spacing(5)
.padding(10)
```

### Keyboard Events

```rust
fn subscription(&self) -> Subscription<Message> {
    iced::event::listen().map(|event| {
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                Message::KeyPressed(key)
            }
            _ => Message::None,
        }
    })
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_creation() {
        let settings = Settings::default();
        let state = AppState::new(&settings);
        assert!(!state.is_connected());
    }
}
```

## Rust Tips for Go Developers

| Go | Rust |
|----|------|
| `err != nil` | `result.is_err()` or `match result { Ok(v) => ..., Err(e) => ... }` |
| `interface{}` | `trait` + `impl Trait` |
| `struct{}` | `struct Name { field: Type }` |
| `go func(){}` | `Task::perform(async { ... }, Message::Result)` |
| `defer` | `Drop` trait or RAII |
| `make([]T, n)` | `Vec::with_capacity(n)` |
| `map[K]V` | `HashMap<K, V>` |

## Current Phase

**Phase 1: Project Scaffolding**
- [x] Cargo project setup
- [x] Basic Iced app structure
- [x] API client with health check
- [x] Config loading
- [ ] Connection status display
- [ ] Unit tests

## Next Steps

Phase 2 will add:
- Stats display
- Navigation state machine
- Breadcrumb component
- Loading indicators
