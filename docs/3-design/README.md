# 3-design

| Document | Description |
|----------|--------------|
| [architecture.md](architecture.md) | Component diagram, why the dependency footprint is exactly `futures`+`thiserror`, why `BrokerFuture`/`Message`'s constructors live in this crate |
| [adr/ADR-001-contract-core-saf-split.md](adr/ADR-001-contract-core-saf-split.md) | Full design record: the original split from `edge-message-broker`, and the amendment flattening to one crate + removing `BackendKind`/`MessageBrokerConfig` |

[← Docs index](../README.md)
