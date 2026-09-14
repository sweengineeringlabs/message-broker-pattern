# 3-design

**Audience**: Architects, technical leads, contributors.

| Document | Description |
|----------|--------------|
| [architecture.md](architecture.md) | Component diagram, why the dependency footprint is `bytes`+`futures`+`thiserror`+`uuid`, why `TaskQueue` lives here alongside `MessageBroker`, why constructors for `BrokerFuture`/`Message`/`Task`-family types live in this crate |
| [compliance/compliance_checklist.md](compliance/compliance_checklist.md) | Architecture compliance checklist derived from architecture.md |
| [adr/README.md](adr/README.md) | ADR index |

[← Docs index](../README.md)
