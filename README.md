# FileMonitor

A lightweight and efficient CLI tool to recursively scan and monitor a directory for file changes using [Rust](https://www.rust-lang.org/) and [tokio](https://tokio.rs/).

---

## Features

- Initial recursive scan of a directory with timestamps
- Real-time monitoring for file system events: `[NEW]`, `[MOD]`, `[DEL]`
- Cross-platform support (Linux, macOS, Windows)
- Graceful Ctrl+C handling with a final snapshot display
- Built without any mutexes, using Tokio async runtime
- Structured error handling using `anyhow`

---

## Run

```bash
git clone https://github.com/sendevman/file_monitor.git
cd file_monitor
cargo run -- -d inbox
