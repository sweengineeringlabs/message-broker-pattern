# message-broker-pattern

> **TLDR:** A reusable message-broker pattern — `MessageBroker`/`Validator` traits, plus
> a zero-external-technology reference implementation. See
> [Architecture](docs/3-design/architecture.md) for the full design.

Pattern crate, not a broker deployment — zero backend-specific technology (no NATS, no
Kafka, no Postgres). A consumer depends on this crate's `saf` for the reference no-op
broker, or on [`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc)
for a real technology-specific backend.

## Quick Start

```rust
use message_broker_pattern_contract::{HealthCheckRequest, MessageBroker};
use message_broker_pattern_saf::BrokerSvc;

let broker = BrokerSvc::noop_broker();
broker.health_check(HealthCheckRequest).await?;
```

## Crates

| Crate | What it is |
|-------|------------|
| [`message-broker-pattern-contract`](scm/main/message-broker/contract) | Trait/type surface only |
| [`message-broker-pattern-core`](scm/main/message-broker/core) | Default `NoopMessageBroker`/`NoopValidator` implementation |
| [`message-broker-pattern-saf`](scm/main/message-broker/saf) | Facade — the recommended entry point for consumers |

Extracted from [`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)
per [edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6),
mirroring this org's existing `<name>-pattern`/`<name>-svc` split (see
[`wasm-capability-pattern`](https://github.com/sweengineeringlabs/wasm-capability-pattern)).
`cargo build/test --workspace`, `cargo fmt --check`, and `cargo clippy --workspace
--all-targets -- -D warnings` all clean.

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, crate boundaries |
| [ADR-001](docs/3-design/adr/ADR-001-contract-core-saf-split.md) | Why this repo is shaped as contract/core/saf |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on a crate |

## License

MIT OR Apache-2.0
