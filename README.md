# Camera Assistant

Camera Assistant (a working title) is a local recorder for the cameras in your home. It takes care of everything that touches video: live view, recording, object detection and playback.

It is built to work with [Home Assistant](https://www.home-assistant.io/) first, and it can also run on its own.

This repository holds the server, written in Rust.

## Status

The project has just started. There is nothing to install yet.

The plan and the open tasks are in the [backlog](https://github.com/camera-assistant/backlog).

## Layout

| Path | What it holds |
|---|---|
| `crates/ca-server/` | The server binary. |
| `crates/ca-<part>/` | One library crate per part of the [architecture](https://github.com/camera-assistant/backlog/blob/main/docs/architecture.md), added by later plans. |
| `modules/` | The open detection modules, one binary each. |
| `conformance/` | The conformance kit that every detection module must pass. |
| `models/` | The manifest of every model, with its licence and checksum. |

## Build

You need [rustup](https://rustup.rs/). It installs the Rust version pinned in `rust-toolchain.toml` on first use.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --workspace --locked
```

The server binary is then in `target/release/ca-server`.

## License

Apache License 2.0. See [LICENSE](LICENSE).
