# message-broker-pattern

## WHAT

Generic, domain-agnostic message-broker pattern: the trait/type surface a broker
consumer is built against (`MessageBroker`, `Validator`), plus this pattern's own
default implementation of the one part that needs no named external technology
(`NoopMessageBroker`/`NoopValidator`).

## WHY

Extracted from [`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)
so the contract can be versioned independently of any backend implementation, and so the
compiler — not just `arch audit` after the fact — enforces that only this repo's `saf`
crate constructs `core` concrete types. See
[edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6)
for the full rationale, and
[`wasm-capability-pattern`](https://github.com/sweengineeringlabs/wasm-capability-pattern)
for the precedent this repo's shape mirrors.

**Status:** contract/core/saf content is ported and passing (`cargo test --workspace`
clean). Architecture diagrams, an ADR documenting the split, and a developer guide are
still open — tracked in this repo's own migration issue (linked from
edge-message-broker#6).
