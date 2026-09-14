# Architecture Decision Records

**Audience**: Architects, technical leads, contributors.

| ADR | Status | Date | Decision |
|-----|--------|------|----------|
| [ADR-001](ADR-001-contract-core-saf-split.md) | Accepted | 2026-09-13 | Split from `edge-message-broker`'s single-crate `api`/`core`/`saf` modules into separate crates, then flattened back to one crate (`core`/`saf` moved out entirely to `message-broker-svc`); `BackendKind`/`MessageBrokerConfig` deleted outright as contract-adjacent vocabulary that must never name a backend technology |
| [ADR-002](ADR-002-split-task-queue-into-its-own-crate.md) | Accepted | 2026-09-14 | Split `TaskQueue` and its own primitive set out into a new `task-queue-pattern` crate (SRP) — `MessageBroker`/`TaskQueue` have different consumers and evolution drivers, bundled originally for migration-completeness, not shared responsibility |

[← 3-design index](../README.md)
