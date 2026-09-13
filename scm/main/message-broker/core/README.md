# message-broker-pattern-core

This pattern's own default implementation: `NoopMessageBroker`/`NoopValidator`, plus
`MessageBrokerConfig` (the `[message_broker]` config vocabulary and its cross-field
validation). No named external technology dependency of its own. A technology-specific
backend (NATS, Kafka, Postgres) belongs in a downstream consumer's own `spi` crate
instead — see [`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc)
once it exists.

Ported from [`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)'s
`core/` module tree per
[edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6).
`MessageBrokerConfig` moved here from `contract` during the port: its `Validator`/
`OptionalSection` impls are real cross-field validation logic (and a real `configbuilder`
dependency), not a zero-implementation contract shape.
