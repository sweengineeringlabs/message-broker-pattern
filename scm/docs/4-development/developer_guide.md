# message-broker-pattern Developer Guide

## Repo Structure

```
message-broker-pattern/
├── README.md
├── scm/
│   ├── Cargo.toml          # workspace: [main/message-broker/{contract,core,saf}]
│   ├── docs/
│   │   ├── README.md                            # WHAT + WHY
│   │   ├── 3-design/architecture.md
│   │   ├── 3-design/adr/ADR-001-contract-core-saf-split.md
│   │   └── 4-development/developer_guide.md     # this file
│   └── main/message-broker/
│       ├── contract/       # message-broker-pattern-contract -- traits/types, zero implementation
│       ├── core/            # message-broker-pattern-core -- NoopMessageBroker/NoopValidator/MessageBrokerConfig
│       └── saf/              # message-broker-pattern-saf -- BrokerSvc facade
```

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Pre-1.0 SemVer: a breaking change bumps the minor version.
- Not yet tagged or published to crates.io — downstream consumers (e.g.
  `message-broker-svc`) resolve this repo via `git`, `branch = "dev"`, until a tag cut
  happens.

## Working on Any Crate

`contract`, `core`, and `saf` are members of `scm/Cargo.toml`, so from `scm/`:

```
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

Or scope to one crate:

```
cargo test --manifest-path main/message-broker/contract/Cargo.toml --all-targets
cargo test --manifest-path main/message-broker/core/Cargo.toml --all-targets
cargo test --manifest-path main/message-broker/saf/Cargo.toml --all-targets
```

`message-broker-pattern-contract` ships zero implementation of `MessageBroker`/
`Validator` and depends only on the crates its trait signatures actually need
(`bytes`, `futures`, `serde`, `thiserror`) — `#![deny(unsafe_code)]` and
`#![warn(missing_docs)]` are enforced across all three crates.

## src/ Layout

`contract`'s `src/` is organized by kind: `traits/`, `vo/`, `dto/`, `error/`, `types/` —
matching `edge-message-broker`'s own `api/` module names it was extracted from (this
repo has only one theme, so no theme-then-kind nesting like `wasm-capability-pattern`
needs). `core` and `saf` are both flat (`core/src/*.rs`, `saf/src/*.rs`).

## No Direct core Import, Anywhere

Enforced, not a style preference: nothing outside `message-broker-pattern-core` itself
should import `message_broker_pattern_core` directly — always go through
`message-broker-pattern-saf`'s `BrokerSvc`. `message-broker-svc`'s own `spi` crates are
the one exception (they need `MessageBrokerConfig` from `core` to populate the value
`validator()` returns) — see that repo's own developer guide.

## The path + version Dependency Rule

`message-broker-pattern-core`/`message-broker-pattern-saf` depend on their siblings with
**both** `path` and `version` set:

```toml
message-broker-pattern-contract = { path = "../contract", version = "0.1.0" }
```

`path` resolves locally in-repo (including pre-release, unpublished changes) — `cargo
publish` strips `path` and resolves purely against the `version` constraint on
crates.io. A `path`-only dependency makes the crate **unpublishable**: `cargo package`
fails outright with "all dependencies must have a version requirement specified when
packaging". Bump every downstream crate's declared version together when an upstream
crate ships a breaking change.

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-contract-core-saf-split.md)
- [Docs index](../README.md)
