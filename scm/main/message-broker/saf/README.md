# message-broker-pattern-saf

Facade for `message-broker-pattern` — the recommended entry point for consumers. `BrokerSvc`
wires `message-broker-pattern-core`'s default implementation behind
`message-broker-pattern-contract`'s traits; a downstream consumer's own `saf`
(`message-broker-svc`) constructs its technology-specific `spi` implementations the same
way.

Ported from [`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)'s
`saf/` module tree per
[edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6).
