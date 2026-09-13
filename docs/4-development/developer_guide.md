# message-broker-pattern Developer Guide

## Repo Structure

```
message-broker-pattern/
├── README.md
├── docs/
│   ├── README.md                                 # docs section index
│   ├── 0-ideation/papers/README.md
│   ├── 3-design/README.md, architecture.md
│   ├── 3-design/adr/ADR-001-contract-core-saf-split.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # [package] message-broker-pattern -- a single crate,
    │                       #  not a workspace
    ├── main/src/
    │   ├── lib.rs
    │   ├── traits/          # MessageBroker, Validator
    │   ├── vo/               # Message
    │   ├── dto/              # *Request/*Response
    │   ├── error/            # BrokerError, ValidationError
    │   └── types/            # BrokerFuture, MessageStream
    └── tests/                 # message_int_test.rs, message_stream_int_test.rs,
                                #  message_broker_int_test.rs, broker_error_int_test.rs
```

Matches `configbuilder`'s own single-crate layout (`[package]` directly in
`scm/Cargo.toml`, `path = "main/src/lib.rs"`, tests at `scm/tests/`) — this repo was a
three-crate `contract`/`core`/`saf` workspace originally; see ADR-001's amendment for
why it collapsed to one.

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Pre-1.0 SemVer: a breaking change bumps the minor version.
- Not yet tagged or published to crates.io — downstream consumers (e.g.
  `message-broker-svc`) resolve this repo via `git`, `branch = "dev"`, until a tag cut
  happens.

## Working on This Crate

```
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Dependency footprint is exactly `futures` (structurally required by `MessageStream`'s
`Stream` bound) and `thiserror` (derive convenience for `BrokerError`/`ValidationError`)
— verify with `cargo tree --depth 1` after any change that touches `Cargo.toml`; if
anything else shows up, it's a regression, not a feature. `#![deny(unsafe_code)]` and
`#![warn(missing_docs)]` are enforced.

## No Backend Vocabulary, Ever

Enforced by design, not just convention: this crate must never gain a type that names a
specific backend technology (an enum listing `Nats`/`Kafka`/`Postgres`, a field named
after one backend's own concept like `group_id`/`queue_name`, anything of that shape).
That vocabulary — `BackendKind`, `MessageBrokerConfig` — lives entirely in
`message-broker-svc-core` instead, and stays a real, zero-cost `enum`/`struct` there,
not a genericized `String`/`HashMap` stand-in kept here. See ADR-001's amendment for the
full reasoning: the set of backends is closed and known by whoever builds `-svc`, which
is exactly when a real enum beats a generic, stringly-typed placeholder.

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-contract-core-saf-split.md)
- [Docs index](../README.md)
