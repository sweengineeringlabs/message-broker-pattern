# message-broker-pattern Architecture

**Audience**: Architects, technical leads, contributors.

## Overview

One crate, `message-broker-pattern` — this domain's one primitive, zero
implementation, zero knowledge of any backend technology:

- **`traits/`** — `MessageBroker` (`publish`/`subscribe`/`health_check`/`validator`),
  `Validator` (`validate(request)` — backend config
  validation — plus two default methods, `validate_config`/`validator_response`, built
  purely from `validate` and this crate's own types; see "Why `validate_config`/
  `validator_response` are default methods on `Validator` itself" below).
- **`vo/`** — `Message` (payload + headers, the currency `MessageBroker` passes around).
- **`dto/`** — `PublishRequest`/`SubscribeRequest`/`SubscribeResponse`/
  `HealthCheckRequest`/`ValidatorRequest`/`ValidatorResponse`/`ValidationRequest`.
- **`error/`** — `BrokerError`, `ValidationError`.
- **`types/`** — `MessageStream` (needs `futures::Stream` directly, since Rust's
  `std` doesn't stabilize a stream trait yet). `MessageBroker`'s own methods
  return `impl Future` directly (RPITIT) rather than a local future wrapper —
  see "Why `MessageBroker` returns `impl Future`, not `BrokerFuture`" below.

`TaskQueue` (competing-consumer work queue — a different delivery semantic,
different consumers, different evolution drivers from `MessageBroker`'s own
fan-out/broadcast) lives in a separate crate,
[`task-queue-pattern`](https://github.com/sweengineeringlabs/task-queue-pattern)
— see "Why `TaskQueue` moved out (SRP)" below.

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
        traits["traits<br/>MessageBroker, Validator"]
        vo["vo<br/>Message"]
        dto["dto<br/>*Request/*Response"]
        error["error<br/>BrokerError, ValidationError"]
        types["types<br/>MessageStream"]
    end

    subgraph svc["message-broker-svc"]
        core["message-broker-svc-core<br/>InMemoryMessageBroker (technology-free, not an spi)"]
        spi["message-broker-svc-{nats,kafka,postgres}-spi<br/>*MessageBroker"]
        saf["message-broker-svc-saf<br/>MessageBrokerFactory,<br/>NoopMessageBroker, NoopValidator"]
    end

    core -.->|implements| traits
    spi -.->|implements| traits
    core -.->|depends on| vo
    core -.->|depends on| dto
    saf -.->|wires| core
    saf -.->|wires| spi
```

`TaskQueue` and its own primitive set (`Task`/`TaskHandle`/`TaskHandleBuilder`/
`TaskId`/`QueueError`/`PayloadValidator`/`TaskQueueFactoryContract`) live in the
separate `task-queue-pattern`/`task-queue-svc` repo pair — not shown above,
see that repo pair's own architecture doc.

## Why the dependency footprint is `futures` + `thiserror`

A pure trait/type crate isn't obligated to have zero dependencies — only zero
dependencies its type *signatures* don't structurally require. Two things earn their
place:

- **`futures`** — `MessageStream`'s definition (`Pin<Box<dyn Stream<...>>>`) names
  `futures::Stream` directly. `MessageBroker`'s own async methods need
  nothing beyond `std::future::Future` (see "Why `MessageBroker` returns
  `impl Future`, not `BrokerFuture`" below).
- **`thiserror`** — derive-macro convenience for `BrokerError`/
  `ValidationError`'s `Display`/`Error` impls. Boilerplate, not domain logic — the same
  category `wasm-capability-pattern-contract` itself accepts (a hand-written
  `PartialEq`, in that case).

`bytes` and `uuid` (needed only by `Task`/`TaskHandle`/`TaskId`) left with `TaskQueue`
when it moved to `task-queue-pattern` — see "Why `TaskQueue` moved out (SRP)" below.
`serde` (only needed by `BackendKind`, which never lives here) stays out entirely. See
ADR-001's amendment for why `BackendKind`/`MessageBrokerConfig` were removed rather
than genericized.

## Why `MessageBroker` returns `impl Future`, not `BrokerFuture`

Until `message-broker-pattern` v0.3.0, `publish`/`subscribe`/`health_check` each
returned `BrokerFuture<'a, T>`, a local wrapper around
`Pin<Box<dyn Future<Output = T> + Send + 'a>>`. Raised as a real, checked zero-cost
abstraction question in
[message-broker-pattern#3](https://github.com/sweengineeringlabs/message-broker-pattern/issues/3):
a boxed future costs a heap allocation on every single call, on a trait whose whole
point is being called constantly (every publish, every subscribe, every health
check). Rust's return-position `impl Future` in traits (RPITIT, stable since 1.75)
removes that allocation entirely — no box, no vtable, statically dispatched,
inlinable — at a real, accepted cost: `MessageBroker` is no longer object-safe,
there is no `Box<dyn MessageBroker>`/`&dyn MessageBroker` anymore.

That loss is not absorbed silently. `message-broker-svc` had a genuine reason to
want one uniform, runtime-selectable broker type — a deployment picking NATS vs.
Kafka vs. Postgres from config without recompiling — so it resolves this with a
static-dispatch enum (`AnyMessageBroker`, one variant per backend, `match`-and-
delegate) instead of clawing object safety back onto this trait. That keeps the
"one factory, one uniform return type" ergonomics `MessageBrokerFactory`'s callers
already relied on, without paying for a vtable to get it. See that repo's own
`docs/3-design/architecture.md` for the full reasoning and implementation.

## Why `Message` constructors live here, in this crate

`Message` needs an inherent `impl` — its constructors. Rust requires an inherent
impl to live in the same crate as the type it's implementing on. This mirrors
`wasm-capability-pattern-contract`'s own precedent (a hand-written `PartialEq` on
its one `entity` type, in that same crate) — a contract/port crate can hold small,
structural `impl`s; "zero implementation" means it implements none of *its own*
primary traits (`MessageBroker`/`Validator`), not that the `impl` keyword never
appears.

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
(`task-queue-pattern`'s own `TaskQueueFactoryContract`, and the "small
structural `impl`" carve-out in "Why `Message` constructors live here, in
this crate" above) already establishes that "zero
implementation" means "this crate implements none of
`Validator`/`MessageBroker` *for any concrete type*" — not "no default
method body may ever appear in a trait declared here." `validate_config`/
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

## Why `TaskQueue` moved out (SRP)

`TaskQueue` (competing-consumer — each task goes to exactly one worker) and
`MessageBroker` (fan-out/broadcast — every subscriber gets every message)
arrived in this crate from the same source pilot and were bundled together
specifically so "a consumer gets this domain's whole primitive set from one
crate" — see "`TaskQueue`'s belated migration" below for that original
reasoning, preserved as history, not rewritten.

Revisited: that reasoning weighed migration-completeness, not single
responsibility. `MessageBroker` and `TaskQueue` have genuinely different
evolution drivers (pub/sub concerns — topic wildcards, ordering — vs. queue
concerns — visibility timeout, retry, dead-letter handling) and different
consumers (some want broadcast, some want a work queue, not all want
both). Shared origin was never shared responsibility. See
[ADR-002](adr/ADR-002-split-task-queue-into-its-own-crate.md) and
[Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)'s
own "Single Responsibility" section, written to capture this exact decision
generically.

`TaskQueue`, `TaskQueueFactoryContract`, `PayloadValidator`,
`Task`/`TaskHandle`/`TaskHandleBuilder`/`TaskId`, `QueueError`, and the four
marker constants (`VALIDATOR_SVC`, `TASK_QUEUE_FACTORY_CONTRACT_ID`,
`MAX_TASK_PAYLOAD_BYTES`, `TASK_ID_HEADER_KEY`) moved to
[`task-queue-pattern`](https://github.com/sweengineeringlabs/task-queue-pattern)
unchanged — every moved test and the `custom_validator` example were
verified to still pass, with only `use` paths changed. `bytes`/`uuid` left
this crate's own dependency list alongside them (nothing remaining here
needs either). Breaking change, pre-1.0 minor bump: `0.1.3 → 0.2.0`.

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

**Amendment**: `TaskQueue` and everything below no longer live in this crate as of
[ADR-002](adr/ADR-002-split-task-queue-into-its-own-crate.md) — moved to
[`task-queue-pattern`](https://github.com/sweengineeringlabs/task-queue-pattern)
for SRP; see "Why `TaskQueue` moved out (SRP)" above. This section is
preserved as the historical record of how `TaskQueue` first arrived here,
not rewritten to pretend it never did.

`TaskQueue`, `TaskQueueFactoryContract`, `Task`/`TaskHandle`/`TaskHandleBuilder`/
`TaskId`, `QueueError`, the payload-validation trait (`PayloadValidator`, renamed on
arrival — see below), and the marker constants lived here, but didn't from the
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
