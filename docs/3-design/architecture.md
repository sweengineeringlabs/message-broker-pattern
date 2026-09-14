# message-broker-pattern Architecture

**Audience**: Architects, technical leads, contributors.

## Overview

One crate, `message-broker-pattern` — every primitive of this domain, zero
implementation, zero knowledge of any backend technology:

- **`traits/`** — `MessageBroker` (`publish`/`subscribe`/`health_check`/`validator`),
  `TaskQueue` (`enqueue`/`dequeue`/`health_check`), `TaskQueueFactoryContract`
  (`new_task_id`/`build_handle`), `Validator` (`validate(request)` — backend config
  validation — plus two default methods, `validate_config`/`validator_response`, built
  purely from `validate` and this crate's own types; see "Why `validate_config`/
  `validator_response` are default methods on `Validator` itself" below),
  `PayloadValidator` (`validate(&self)` — generic self-validation, distinct
  from `Validator`).
- **`vo/`** — `Message` (payload + headers, the currency `MessageBroker` passes around),
  `Task`/`TaskHandle`/`TaskHandleBuilder`/`TaskId` (the equivalent currency for
  `TaskQueue`).
- **`dto/`** — `PublishRequest`/`SubscribeRequest`/`SubscribeResponse`/
  `HealthCheckRequest`/`ValidatorRequest`/`ValidatorResponse`/`ValidationRequest`.
- **`error/`** — `BrokerError`, `QueueError`, `ValidationError`.
- **`types/`** — `BrokerFuture` (the async return type every `MessageBroker` method
  uses — needs only `std::future::Future`), `MessageStream` (needs `futures::Stream`
  directly, since Rust's `std` doesn't stabilize a stream trait yet), and marker
  constants (`VALIDATOR_SVC`, `TASK_QUEUE_FACTORY_CONTRACT_ID`,
  `MAX_TASK_PAYLOAD_BYTES`, `TASK_ID_HEADER_KEY`).

Everything that would make this crate aware of a specific technology — a no-op
reference implementation, real NATS/Kafka/Postgres backends, and the facade that
dispatches to one — lives in
[`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc)
instead. This repo has no knowledge of, and no dependency on, any of it. Nor does it
know about any specific *application's* composition choices: no config-section name,
no default-backend value, no `ApplicationConfig`-shaped type — a contract crate
declares what a backend *is*, never how a specific application chooses or configures
one.

## Component Diagram

```mermaid
flowchart TD
    subgraph pattern["message-broker-pattern (this repo)"]
        traits["traits<br/>MessageBroker, TaskQueue,<br/>TaskQueueFactoryContract,<br/>Validator, PayloadValidator"]
        vo["vo<br/>Message, Task, TaskHandle,<br/>TaskHandleBuilder, TaskId"]
        dto["dto<br/>*Request/*Response"]
        error["error<br/>BrokerError, QueueError, ValidationError"]
        types["types<br/>BrokerFuture, MessageStream, constants"]
    end

    subgraph svc["message-broker-svc"]
        core["message-broker-svc-core<br/>InMemoryMessageBroker + InMemoryTaskQueue (technology-free, not an spi)"]
        spi["message-broker-svc-{nats,kafka,postgres}-spi<br/>*MessageBroker + *TaskQueue (no postgres TaskQueue)"]
        saf["message-broker-svc-saf<br/>MessageBrokerFactory, TaskQueueFactory,<br/>NoopMessageBroker, NoopValidator"]
    end

    core -.->|implements| traits
    spi -.->|implements| traits
    core -.->|depends on| vo
    core -.->|depends on| dto
    saf -.->|wires| core
    saf -.->|wires| spi
```

## Why the dependency footprint is `bytes` + `futures` + `thiserror` + `uuid`

A pure trait/type crate isn't obligated to have zero dependencies — only zero
dependencies its type *signatures* don't structurally require. Four things earn their
place:

- **`bytes`** — `Task`/`TaskHandle`/`TaskHandleBuilder`'s `payload` field is `Bytes`
  directly (not retyped to `impl Into<Vec<u8>>`), matching how `TaskQueue`
  implementations actually carry payloads.
- **`futures`** — `MessageStream`'s definition (`Pin<Box<dyn Stream<...>>>`) names
  `futures::Stream` directly, and `TaskHandle`/`TaskHandleBuilder`'s `ack`/`nack`
  fields are `BoxFuture<'static, Result<(), QueueError>>`. `BrokerFuture` itself needs
  nothing beyond `std::future::Future`.
- **`thiserror`** — derive-macro convenience for `BrokerError`/`QueueError`/
  `ValidationError`'s `Display`/`Error` impls. Boilerplate, not domain logic — the same
  category `wasm-capability-pattern-contract` itself accepts (a hand-written
  `PartialEq`, in that case).
- **`uuid`** — `TaskId` wraps `Uuid` directly and generates one via `Uuid::new_v4()`.

`serde` (only needed by `BackendKind`, which never lives here) stays out entirely. See
ADR-001's amendment for why `BackendKind`/`MessageBrokerConfig` were removed rather
than genericized.

## Why `BrokerFuture`/`Message`/`Task`-family constructors live here, in this crate

`BrokerFuture<'a, T>`, `Message`, `TaskId`, `Task`, `TaskHandle`, and
`TaskHandleBuilder` each need an inherent `impl` — constructors, `Display`, and
`BrokerFuture`'s own `Future` impl. Rust requires an inherent impl to live in the same
crate as the type it's implementing on. This mirrors `wasm-capability-pattern-contract`'s
own precedent (a hand-written `PartialEq` on its one `entity` type, in that same crate)
— a contract/port crate can hold small, structural `impl`s; "zero implementation" means
it implements none of *its own* primary traits (`MessageBroker`/`TaskQueue`/
`Validator`/`PayloadValidator`), not that the `impl` keyword never appears.

## Why `validate_config`/`validator_response` are default methods on `Validator` itself

Every backend's `MessageBroker::validator()` implementation
(`NatsMessageBroker`, `KafkaMessageBroker`, `PostgresMessageBroker`,
`InMemoryMessageBroker`, `NoopMessageBroker`, all in `message-broker-svc`)
needs the same two operations: map its own `Validator::validate` result into
a `BrokerError` before doing any I/O, and wrap `Arc<Self>` as a type-erased
`ValidatorResponse`. Every `Validator` implementor's own `validate` body is
genuinely backend-specific (that stays a required method), but the two
operations built on top of it are byte-for-byte identical for every
implementor and touch nothing but `validate` itself plus this crate's own
`BrokerError`/`ValidationRequest`/`ValidatorResponse` — no backend-technology
vocabulary, no external dependency.

That made them `message-broker-svc-spi-shared`'s `ValidatorExt` at first: an
extension trait, kept out of this crate on the assumption that "zero
implementation" ruled out defining any real logic here. That assumption
didn't hold once the logic was checked: this crate's own precedent
(`TaskQueueFactoryContract`, and the "small structural `impl`" carve-out in
"Why `BrokerFuture`/`Message`/`Task`-family constructors live here, in this
crate" above) already establishes that "zero implementation" means "this
crate implements none of `Validator`/`MessageBroker`/`TaskQueue`/
`PayloadValidator` *for any concrete type*" — not "no default method body
may ever appear in a trait declared here." `validate_config`/
`validator_response` don't implement `Validator` for anything; they're
default methods *on* `Validator`, exactly the same shape `Iterator::count`
is a default method built from `next` in `std`. Rust supports this cleanly:
`validator_response(self: &Arc<Self>) -> ValidatorResponse where Self: Sized
+ 'static` is excluded from `Validator`'s vtable by its own `Sized` bound,
so `Arc<dyn Validator>` (used by `ValidatorResponse` itself) keeps working
unmodified.

Moved here directly (`message-broker-pattern` v0.1.3) rather than as a
separate `ValidatorExt` trait — no extension trait needed once the methods
live on `Validator` itself, so `message-broker-svc-spi-shared` was deleted
outright, not kept as a re-export. See
[message-broker-svc#3](https://github.com/sweengineeringlabs/message-broker-svc/issues/3)
for the review this settled, and that repo's own architecture doc for the
consumer-side half of this change.

## History

Ported from `edge-message-broker`'s single-crate `main/src/{api,core,saf}` module tree
per [edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6),
initially as a three-crate `contract`/`core`/`saf` split mirroring `wasm-capability-pattern`.
Flattened to this single crate, with `core`/`saf`/`BackendKind`/`MessageBrokerConfig` all
moved into `message-broker-svc`, per ADR-001's amendment — see that document for the full
reasoning.

`spi/broker_backend.rs`'s `BrokerBackend` marker type was never ported at all: it was
`pub(crate)` in the original single crate, referenced nowhere outside its own file, and
its own test file didn't actually exercise it either — dead code kept alive only by a
`dead_code`-lint workaround.

## `TaskQueue`'s belated migration

`TaskQueue`, `TaskQueueFactoryContract`, `Task`/`TaskHandle`/`TaskHandleBuilder`/
`TaskId`, `QueueError`, the payload-validation trait (`PayloadValidator`, renamed on
arrival — see below), and the marker constants now live here, but didn't from the
start. `edge-runtime`'s own port of this crate's original content
(`runtime-message-broker-contract`, later renamed `-pattern`) is a superset that also
defines `TaskQueue` — a distinct competing-consumer work-queue contract with no
`MessageBroker` equivalent — but that half was left behind in `edge-runtime` instead of
migrating here alongside `MessageBroker`. That was an incomplete migration, not a
deliberate scope boundary: a consumer is supposed to get this domain's whole primitive
set from this crate plus `message-broker-svc`, not half of it redefined downstream in
every consuming repo. Ported into this crate directly from `edge-runtime`'s copy,
unchanged apart from one necessary rename: `edge-runtime`'s own `Validator` trait
(`fn validate(&self) -> Result<(), String>`, a generic self-check) collided by name
with this crate's own `Validator` (`fn validate(&self, request: ValidationRequest) ->
Result<(), ValidationError>`, backend config validation) — genuinely different
contracts that happened to share a name across two crates that had never been merged
before. Renamed to `PayloadValidator` on arrival to resolve the collision; its
signature and semantics are otherwise untouched.

`message-broker-svc` now implements `TaskQueue` too:
`message-broker-svc-{inmemory,nats,kafka}-spi` each ship a `*TaskQueue` alongside
their `*MessageBroker` (no `postgres` — that backend never had one), and
`message-broker-svc-saf`'s `TaskQueueFactory` constructs them, mirroring
`MessageBrokerFactory`'s own shape — see that repo's own architecture doc. With
that landed, `edge-runtime`'s local `core`/`spi/{kafka,nats}` crates held nothing
but duplicated re-exports of primitives and backends that now live upstream, so
they were deleted outright rather than kept as pass-throughs.
`runtime-message-broker-saf` is the only crate `edge-runtime` has left in this
domain, depending on this crate and `message-broker-svc-*-spi` directly — a
consumer gets this domain's whole primitive set and every real backend from two
published repos, none of it redefined downstream.

[← Docs index](../README.md)
