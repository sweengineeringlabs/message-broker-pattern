# message-broker-pattern Developer Guide

**Audience**: Developers, contributors.

## Repo Structure

```
message-broker-pattern/
├── README.md
├── docs/
│   ├── README.md                                 # docs section index
│   ├── glossary.md                                # domain terminology
│   ├── 0-ideation/papers/README.md
│   ├── 3-design/README.md, architecture.md
│   ├── 3-design/compliance/compliance_checklist.md
│   ├── 3-design/adr/README.md, ADR-001-contract-core-saf-split.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # [package] message-broker-pattern -- a single crate,
    │                       #  not a workspace
    ├── examples/            # custom_validator.rs
    ├── src/
    │   ├── lib.rs
    │   ├── traits/          # MessageBroker, TaskQueue, TaskQueueFactoryContract,
    │   │                     #  Validator, PayloadValidator
    │   ├── vo/               # Message, Task, TaskHandle, TaskHandleBuilder, TaskId
    │   ├── dto/              # *Request/*Response
    │   ├── error/            # BrokerError, QueueError, ValidationError
    │   └── types/            # BrokerFuture, MessageStream, marker constants
    └── tests/                 # one *_int_test.rs per public trait/type
```

`src/` and `tests/` are direct siblings under `scm/` — no `main/` intermediate directory
(flattened further than `configbuilder`'s own `main/src/` layout; see ADR-001's amendment
for why). This repo was a three-crate `contract`/`core`/`saf` workspace originally; see
ADR-001's first amendment for why it collapsed to one.

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Pre-1.0 SemVer: a breaking change bumps the minor version.
- Published to crates.io as [`message-broker-pattern`](https://crates.io/crates/message-broker-pattern),
  currently v0.1.2 (`TaskQueue`/`PayloadValidator` and the `custom_validator` example were
  purely additive 0.1.0 → 0.1.2 bumps), tagged to match in this repo's own git history.
  Downstream consumers (e.g. `message-broker-svc`) depend on it by version, not `git`.

## Working on This Crate

```
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Dependency footprint is `bytes` (`Task`/`TaskHandle`/`TaskHandleBuilder`'s `payload`
field), `futures` (structurally required by `MessageStream`'s `Stream` bound and
`TaskHandle`'s `ack`/`nack` futures), `thiserror` (derive convenience for
`BrokerError`/`QueueError`/`ValidationError`), and `uuid` (`TaskId` wraps `Uuid`
directly) — verify with `cargo tree --depth 1` after any change that touches
`Cargo.toml`; if anything else shows up, it's a regression, not a feature.
`#![deny(unsafe_code)]` and `#![warn(missing_docs)]` are enforced.

## No Backend Vocabulary, Ever

Enforced by design, not just convention: this crate must never gain a type that names a
specific backend technology (an enum listing `Nats`/`Kafka`/`Postgres`, a field named
after one backend's own concept like `group_id`/`queue_name`, anything of that shape).
The original `BackendKind`/`MessageBrokerConfig` vocabulary was deleted outright, not
relocated — each `message-broker-svc` `spi` crate now defines its own local config type
(`NatsConfig`/`KafkaConfig`/`PostgresConfig`/`InMemoryConfig`), independently
implementing this crate's own `Validator`. No shared config/backend-selector type exists
anywhere in either repo. See `message-broker-svc`'s own ADR-001 amendment for the full
reasoning: the set of backends is closed and known by whoever builds `-svc`, which is
exactly when a real enum would beat a generic, stringly-typed placeholder — but the
right call was recognizing this vocabulary belongs entirely on the tech-aware side of
the boundary, one independent type per backend, not a single shared type anywhere.

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-contract-core-saf-split.md)
- [Docs index](../README.md)
