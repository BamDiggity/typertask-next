# TyperTask-Next

High-performance, Rust-powered, API-first, and Tailscale-native Windows 11 automation, macro playback, and real-time text expansion engine designed for both human power users and autonomous AI agents.

## Features

- **Robust Win32 Engine:** Native Win32 `SendInput` and `WH_KEYBOARD_LL` global keyboard hooks for precise input injection and zero-latency text expansion.
- **Axum Daemon API:** Secure REST control plane (`POST /api/v1/execute`) designed for local scripts and Tailscale-connected AI agents.
- **Plain-Text Macro Syntax:** Simple, readable script files (`.tt`) supporting delays, clicks, mouse movement, keystrokes, and repetition loops.
- **Tailscale Native:** Binds securely to your private WireGuard mesh network for zero public exposure.

## Quickstart

1. Clone or download `typertask-next.exe`.
2. Run daemon mode:
   ```cmd
   typertask-next serve --bind 100.x.y.z:8080
   ```
3. Execute a script via CLI or API:
   ```cmd
   typertask-next run --file macros/example.tt
   ```

## Documentation

- [API Specification](docs/API.md)
- [Macro Script Syntax Reference](docs/SYNTAX.md)
