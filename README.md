# message-broker-pattern

> **TLDR:** The reusable message-broker pattern — every primitive of this domain,
> `MessageBroker` and `TaskQueue` alike, zero implementation, zero knowledge of any
> backend technology. See [Architecture](docs/3-design/architecture.md) for the full
> design.

A single crate, not a workspace, not a broker deployment. Implement `MessageBroker`/
`TaskQueue` to plug in any backend; this repo ships neither — see
[`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc) for a
no-op reference broker and real NATS/Kafka/Postgres backends. A consumer depends on this
crate plus `message-broker-svc` and defines no primitives of its own.

## Quick Start

```rust
use message_broker_pattern::{HealthCheckRequest, MessageBroker};

async fn check(broker: &dyn MessageBroker) -> Result<(), message_broker_pattern::BrokerError> {
    broker.health_check(HealthCheckRequest).await
}
```

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-core-saf-split.md) | Why this repo is a single crate, why BackendKind isn't here |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
