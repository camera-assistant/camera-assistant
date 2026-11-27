# Instructions for coding agents

Read this before you change anything in this repository.

## What this repository is

The server of Camera Assistant (a working title): a local recorder for the cameras in a home, written in Rust. It also holds the open detection modules and the conformance kit that every detection module must pass.

Read the [architecture](https://github.com/camera-assistant/backlog/blob/main/docs/architecture.md) first. It names the parts of the server and how they fit together.

## Plans

The plans live in the backlog repository, in [`docs/plans/`](https://github.com/camera-assistant/backlog/tree/main/docs/plans). Each issue belongs to a step of a plan. Read the plan step of your issue before you write code.

## Build, test and check

CI runs exactly these commands. Run them before you open a pull request; all of them must pass.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --workspace --locked
```

CI builds for `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`. The compiler version is pinned in `rust-toolchain.toml`; rustup installs it on first use.

## Layout

| Path | What it holds |
|---|---|
| `crates/ca-server/` | The server binary: configuration, startup, supervision of the parts. |
| `crates/ca-<part>/` | One library crate per part of the architecture, such as `ca-ingest` or `ca-recorder`. Later plans add them. |
| `modules/ca-detect-<target>/` | One binary per open detection module. |
| `conformance/` | The conformance kit: reference cases and the runner. |
| `models/` | The model manifest: source, licence and sha256 of every model. |

The workspace includes `crates/*` and `modules/*` by glob, so a new crate there needs no change to the root `Cargo.toml`. Every folder in `crates/` and `modules/` must hold a `Cargo.toml`.

## Conventions

- **Workspace settings.** A crate inherits the shared package fields and lints (`lints.workspace = true`). Every dependency is declared once in `[workspace.dependencies]` of the root `Cargo.toml`, and a crate uses it with `dep = { workspace = true }`.
- **No `unsafe`.** `unsafe_code` is forbidden in every crate, except a crate that wraps a C library. Such a crate is named `*-sys` or is a hardware module, sets its own lints, and says in its README why it needs `unsafe`.
- **Warnings fail CI.** Fix the cause.
- **Tests come with the change**, next to the code they test. A bug fix comes with a test that fails without the fix.
- **Error handling and logging.** The crates for these are not chosen yet. The plan step of your issue decides; do not pick one on your own.
- **No secrets in logs.** Log lines, errors and API answers never contain camera passwords, tokens or stream addresses with credentials in them.
- **Dependency licences.** A new external dependency must be under Apache-2.0, MIT, BSD-2-Clause, BSD-3-Clause, ISC, Zlib or Unicode-3.0, and come from crates.io. Crates of this workspace depend on each other by path. Models must be Apache-2.0 or MIT and have an entry in `models/models.toml`. FFmpeg is only used as an LGPL build. Closed vendor runtimes never enter this repository.

## What not to do

- No changes outside the scope of your issue.
- No hand edits to generated files. `Cargo.lock` changes only through cargo, and is committed with the change that causes it.
- No disabled lints, ignored tests or `#[allow(...)]` to make CI pass.
- No secrets, real camera addresses or real credentials in code, tests or fixtures.

## When to stop and ask

Stop and ask before you go on when:

- the plan does not answer a design question you run into;
- your change touches the API schema.

## Pull requests

- One issue per pull request, and keep it small.
- Link the issue with its full address, such as `Closes https://github.com/camera-assistant/backlog/issues/<n>`, and name the plan step.
- The person who opens the pull request has read and run the change, and answers for it.

The full contribution rules are in [CONTRIBUTING.md](https://github.com/camera-assistant/.github/blob/main/CONTRIBUTING.md).
