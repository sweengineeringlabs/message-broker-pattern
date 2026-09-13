# message-broker-pattern

Generic, domain-agnostic message-broker pattern: the `MessageBroker`/`Validator` traits,
plus this pattern's own default implementation of the one part that needs no named
external technology.

Extracted from [`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)
per [edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6),
mirroring this org's existing `<name>-pattern`/`<name>-svc` split (see
[`wasm-capability-pattern`](https://github.com/sweengineeringlabs/wasm-capability-pattern)).

Three crates:

| Crate | What it is |
|-------|------------|
| [`message-broker-pattern-contract`](scm/main/message-broker/contract) | Trait/type surface only |
| [`message-broker-pattern-core`](scm/main/message-broker/core) | Default `NoopMessageBroker`/`NoopValidator` implementation |
| [`message-broker-pattern-saf`](scm/main/message-broker/saf) | Facade — the recommended entry point for consumers |

A technology-specific implementation (NATS, Kafka, Postgres) belongs in
[`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc) instead
— this repo has no knowledge of, and no dependency on, any of its consumers.

**Status:** scaffold only — real trait/type content is being ported from
`edge-message-broker`. Track progress via this repo's migration issue (linked from
[edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6)).

## Documentation

| Document | Description |
|----------|--------------|
| [Overview](scm/docs/README.md) | WHAT + WHY |
| [Architecture](scm/docs/3-design/architecture.md) | Component diagram, crate boundaries |

## License

MIT OR Apache-2.0
