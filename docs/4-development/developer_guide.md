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
│   ├── 3-design/adr/README.md, ADR-001-contract-core-saf-split.md,
│   │                     ADR-002-split-task-queue-into-its-own-crate.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # [package] message-broker-pattern -- a single crate,
    │                       #  not a workspace
    ├── src/
    │   ├── lib.rs
    │   ├── traits/          # MessageBroker, Validator
    │   ├── vo/               # Message
    │   ├── dto/              # *Request/*Response
    │   ├── error/            # BrokerError, ValidationError
    │   └── types/            # MessageStream
    └── tests/                 # one *_int_test.rs per public trait/type
```

No `examples/` directory — `custom_validator.rs` moved to `task-queue-pattern`
alongside `PayloadValidator`/`TaskHandleBuilder`, the primitives it
demonstrated.

`src/` and `tests/` are direct siblings under `scm/` — no `main/` intermediate directory
(flattened further than `configbuilder`'s own `main/src/` layout; see ADR-001's amendment
for why). This repo was a three-crate `contract`/`core`/`saf` workspace originally; see
ADR-001's first amendment for why it collapsed to one.

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Pre-1.0 SemVer: a breaking change bumps the minor version.
- Currently v0.2.0 (git-tagged; not yet published to crates.io at this version —
  the last published version, v0.1.2, predates both `Validator::validate_config`/
  `validator_response` and this version's breaking `TaskQueue` removal).
  `TaskQueue`/`PayloadValidator` and the `custom_validator` example were purely
  additive `0.1.0 → 0.1.2` bumps; `Validator::validate_config`/`validator_response`
  as default methods (moved from `message-broker-svc-spi-shared`) was another purely
  additive bump, `0.1.2 → 0.1.3`; splitting `TaskQueue` back out into
  `task-queue-pattern` (see [ADR-002](../3-design/adr/ADR-002-split-task-queue-into-its-own-crate.md))
  is this crate's first *breaking* change, `0.1.3 → 0.2.0`. Downstream consumers
  needing `TaskQueue` now depend on `task-queue-pattern` separately.

## Working on This Crate

```
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Dependency footprint is `futures` (structurally required by `MessageStream`'s
`Stream` bound) and `thiserror` (derive convenience for
`BrokerError`/`ValidationError`) — verify with `cargo tree --depth 1` after any
change that touches `Cargo.toml`; if anything else shows up, it's a regression,
not a feature. `bytes`/`uuid` left with `TaskQueue` when it moved to
`task-queue-pattern`. `#![deny(unsafe_code)]` and `#![warn(missing_docs)]` are
enforced.

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
- [ADR-002](../3-design/adr/ADR-002-split-task-queue-into-its-own-crate.md)
- [Docs index](../README.md)
- [Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)
