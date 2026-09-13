# message-broker-pattern-core

This pattern's own default implementation: `NoopMessageBroker`/`NoopValidator`, with no
named external technology dependency. A technology-specific backend (NATS, Kafka,
Postgres) belongs in a downstream consumer's own `spi` crate instead — see
[`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc) once it
exists.

**Status:** placeholder scaffold — real content is being ported from
[`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)'s
`core/` module tree. See this repo's migration issue (linked from
[edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6)).
