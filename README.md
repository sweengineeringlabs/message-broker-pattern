# message-broker-pattern

> **TLDR:** The reusable message-broker pattern — one primitive, `MessageBroker`
> (fan-out/broadcast), zero implementation, zero knowledge of any backend
> technology. See [Architecture](docs/3-design/architecture.md) for the full
> design.

A single crate, not a workspace, not a broker deployment. Implement `MessageBroker`
to plug in any backend; this repo ships none — see
[`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc) for a
no-op reference broker and real NATS/Kafka/Postgres backends. A consumer depends on this
crate plus `message-broker-svc` and defines no primitives of its own.

`TaskQueue` (competing-consumer work queue) lives in the separate
[`task-queue-pattern`](https://github.com/sweengineeringlabs/task-queue-pattern)/
[`task-queue-svc`](https://github.com/sweengineeringlabs/task-queue-svc) repo
pair — split out for SRP, see
[ADR-002](docs/3-design/adr/ADR-002-split-task-queue-into-its-own-crate.md).

## Quick Start

```rust
use message_broker_pattern::{HealthCheckRequest, MessageBroker};

async fn check(broker: &impl MessageBroker) -> Result<(), message_broker_pattern::BrokerError> {
    broker.health_check(HealthCheckRequest).await
}
```

`&impl MessageBroker`, not `&dyn MessageBroker` — `MessageBroker`'s methods
return `impl Future` (zero-cost, no boxed future per call), which means the
trait is not object-safe. See [Architecture](docs/3-design/architecture.md)'s
"Why `MessageBroker` returns `impl Future`, not `BrokerFuture`".

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-core-saf-split.md) | Why this repo is a single crate, why BackendKind isn't here |
| [ADR-002](docs/3-design/adr/ADR-002-split-task-queue-into-its-own-crate.md) | Why `TaskQueue` moved to a separate repo pair (SRP) |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
