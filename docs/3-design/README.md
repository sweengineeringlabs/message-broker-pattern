# 3-design

**Audience**: Architects, technical leads, contributors.

| Document | Description |
|----------|--------------|
| [architecture.md](architecture.md) | Component diagram, why the dependency footprint is `futures`+`thiserror`, why `TaskQueue` moved out to `task-queue-pattern` (SRP), why `MessageBroker` returns `impl Future` not a boxed future, why `Message`'s constructors live in this crate |
| [compliance/compliance_checklist.md](compliance/compliance_checklist.md) | Architecture compliance checklist derived from architecture.md |
| [adr/README.md](adr/README.md) | ADR index |

[← Docs index](../README.md)
