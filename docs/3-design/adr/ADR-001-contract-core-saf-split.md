# ADR-001: Split into contract, core, and saf crates

**Status**: Accepted
**Date**: 2026-09-13
**Deciders**: Amu Hlongwane

## Context

`edge-message-broker` shipped `api`/`core`/`saf` as **modules** inside one crate
(`swe-edge-message-broker`), not separate crates. `arch audit` could only check the
module boundary by source path, after the fact — nothing stopped a caller who already
depends on that one crate from reaching past `saf::` into `core::` internals, since
they compile into the same crate anyway.

This org's own established precedent keeps a generic, backend-agnostic contract +
reference implementation in its own multi-crate `<name>-pattern` repo, each SEA layer a
separate Cargo workspace member, so the compiler enforces the boundary instead:
`wasm-capability-pattern` (`contract`/`core`/`saf`), `svc-registry`→`svc-registry-pattern`
(`port`/`adapter`). `edge-message-broker#6` decided to give this crate the same
treatment, extracting it into this repo (mirroring `wasm-capability-pattern`) plus a
sibling `message-broker-svc` repo for the technology-specific backends (mirroring
`wasm-capability-svc`).

## Decision

- `scm/main/message-broker/{contract,core,saf}`, one Cargo workspace member each,
  `scm/Cargo.toml` as the workspace root — matching `wasm-capability-pattern`'s own
  layout exactly (crate-name pattern, `application_type` values, docs shape).
- `application_type` values: `contract` → `"port"`, `core` → `"adapter"`, `saf` →
  `"facade"` — not the newer `"contract"`/`"core"`/`"saf"` vocabulary `cli-pattern` and
  `edge-runtime#96` use, because the locally installed `arch` version (`0.3.56`) only
  recognizes `lib, binary, gateway, shell_binary, shell_lib, web, port, adapter, facade,
  composition_root` as valid `application_type` values (confirmed via `arch explain
  application_type_defined`) — `wasm-capability-pattern`'s own crates already use
  `port`/`adapter`/`facade` against this same `arch` version, so this matches real,
  currently-verifiable precedent rather than a newer vocabulary this installation can't
  validate.
- `MessageBrokerConfig` moved from `edge-message-broker`'s `api/types/` to this repo's
  `core/`, not `contract/` — its `Validator`/`OptionalSection` impls are real
  cross-field validation logic and a real `configbuilder` dependency. `BrokerFuture` and
  `Message`'s constructors stayed in `contract/` alongside their struct declarations —
  seen `architecture.md`'s own section on this for the Rust-level reason (inherent impls
  must live in the same crate as their type).
- `spi/broker_backend.rs`'s `BrokerBackend` marker was not ported — verified dead (see
  `architecture.md`).

## Consequences

- Breaking rename for anything currently depending on `edge-message-broker`
  (`swe-edge-message-broker`) — `edge-message-broker#6`'s own E5 covers archiving that
  repo with a migration pointer, not resolved here.
- `cargo tree` from `core`/`saf` confirms no dependency on a sibling implementation
  crate outside its own layer: `core` depends only on `contract`; `saf` depends on
  `contract` + `core`.
- Known, accepted `arch audit --rs` exceptions (verified empirically against
  `wasm-capability-pattern-core`/`-saf` with the same `arch` version, not unique to this
  repo): `package_name_no_sea_suffix`/`boundary_no_antipattern_names` fire on the
  `-core`/`-saf` crate name suffixes themselves (an org-wide naming convention this
  `arch` version's rule predates); `deps_have_integration_tests` fires on `saf`'s
  dependency on `core` (no test directly names `message-broker-pattern-core`, only
  exercises it through `BrokerSvc`); `security_dependency_audit_configured` is a
  subdirectory-scoped audit artifact (the root `deny.toml` exists but isn't seen when
  auditing a nested crate path in isolation); `examples_dir_lib` is info-severity per
  `arch explain` and non-blocking.

## Verification

`cargo build/test --workspace` clean (49 tests + 1 doctest — the original 51 minus
`BrokerBackend`'s 2 dead-coverage tests). `cargo fmt --check` and `cargo clippy
--workspace --all-targets -- -D warnings` both clean.

## Related patterns

- **Port and Adapter** (Cockburn) — the trait/implementation split itself.
- **Facade** (GoF) — `BrokerSvc`'s role as the sole construction seam.

## Amendment: 2026-09-13 -- flatten to a single crate, remove BackendKind/MessageBrokerConfig

This repo's original three-crate split (`contract`/`core`/`saf`) mislabeled this
repo itself. "Contract" is a *role* a piece of code plays -- trait signatures plus
the minimal vocabulary needed to state them, zero implementation of its own traits --
not a layer name a crate needs alongside sibling `core`/`saf` layers. Bundling all
three together *inside one repo* made this repo look like a smaller SEA stack, when
its actual job is to *be* the contract, in full, on its own -- the reference
implementation (`core`) and the facade/dispatch (`saf`) are composition concerns
belonging to whatever *offers multiple implementations to choose from*, which is
`message-broker-svc`, not this repo.

**Decision**:
- Collapsed `contract`/`core`/`saf` into one crate, `message-broker-pattern`
  (`main/src/{traits,vo,dto,error,types}/`, `tests/` -- both now direct children of
  `scm/`, matching `configbuilder`'s own single-crate layout). `core` and `saf`'s
  entire content (`NoopMessageBroker`, `NoopValidator`, `BrokerSvc`) moved into
  `message-broker-svc`, merged with its existing `saf`.
- `BackendKind` and `MessageBrokerConfig` moved out entirely too, into
  `message-broker-svc-core` -- not genericized and kept, removed outright. Neither
  `MessageBroker` nor `Validator` ever referenced either type (verified by grep
  across every trait/DTO/error file in this crate before the move: zero hits).
  `BackendKind`'s variants (`InMemory`/`Nats`/`Kafka`/`Postgres`) name specific
  technologies directly -- the same defect `wasm-capability-pattern`'s own
  `CapabilityProtocol` enum had before its ADR-001 amendment deleted it outright
  ("a closed six-variant enum matching wasm-capability-svc's own six -spi crates
  exactly... confirmed dead weight, not just domain-specific"). A contract must
  never know a specific technology exists; `runtime-svc-registry`'s `RuntimePhase`
  enum shows the alternative that *is* legitimate in a port crate -- variants
  describing abstract lifecycle *states* (`Configured`/`Starting`/`Running`/...),
  never implementation names.
  - Considered, rejected: genericizing `BackendKind` into a `String`-keyed type plus
    a `HashMap<String, String>` options bag, to keep *some* form of it here. Rejected
    on zero-cost-abstraction grounds, not just naming: the set of backends is closed
    and known by whoever builds `message-broker-svc` (unlike `Registry<T: Named>`'s
    truly open-ended `T`, which is *why* `Named::name()` accepting a runtime `&str`
    is the right tradeoff there) -- a real `enum` (jump table, compiler-checked
    exhaustiveness) is strictly better than a `String` match for a closed set, and
    forcing genericity onto something that doesn't need it just trades zero-cost for
    flexibility this domain will never use. The right fix wasn't watering the type
    down, it was recognizing it belongs entirely on the tech-aware side of the
    boundary, at full strength, not in this repo at all.
- `Message::new`/`Message::with_headers` retyped from `impl Into<Bytes>` to
  `impl Into<Vec<u8>>` -- `bytes` was a real but avoidable dependency (`Message.payload`
  is already `Vec<u8>`; the `Bytes` middle step bought nothing). `serde` dropped
  entirely -- its only use in this crate was `BackendKind`'s `Deserialize` derive,
  which left with `BackendKind`.

**Consequences**:
- Dependency count: 4 (`bytes`, `futures`, `serde`, `thiserror`) -> 2 (`futures`,
  `thiserror`). `futures` stays because `MessageStream`'s definition names
  `futures::Stream` directly -- structurally required by the type signature itself,
  not implementation. `thiserror` stays as derive-macro convenience for
  `BrokerError`/`ValidationError`'s `Display`/`Error` impls -- boilerplate, not
  domain logic, same category `wasm-capability-pattern-contract` itself accepts
  (hand-written `PartialEq`, in that case).
- `message-broker-svc`'s existing `saf` (`MessageBrokerFactory`) absorbs `BrokerSvc`'s
  methods (`noop_broker`, `create_validator`, `validate`, `create_config_builder`) --
  one facade type in that repo, not two.
- Every `spi` crate now depends on `message-broker-svc-core` (for
  `MessageBrokerConfig`) in addition to `message-broker-pattern` (for the traits) --
  tracked in `message-broker-svc`'s own ADR-001.

Verified: `cargo tree --depth 1` shows exactly `futures` + `thiserror`, nothing else.
`cargo build/test` clean (9 tests + 1 doctest -- the original 15 minus
`backend_kind_int_test.rs`'s 6, moved with `BackendKind`). `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` both clean.

## Amendment: 2026-09-13 -- remove `MessageBroker::validator`, `ValidatorRequest`/`ValidatorResponse`

While reviewing this design against `ledger`'s `LedgerPayload` (a trait declared in
`ledger-base-port`, with every leaf port trait -- `LedgerWriter<P>`/`LedgerReader<P>`/
`LedgerVerifier<P>` -- generic over `P: LedgerPayload`, so the data flowing through
every real operation is required, at compile time, to satisfy a trait the contract
itself owns), a mismatch surfaced in this crate's own `MessageBroker::validator`
method, added when `NatsConfig`/`KafkaConfig`/`PostgresConfig` were designed (see
`message-broker-svc`'s own ADR-001 amendment): a `validator(&self, ...) ->
Result<ValidatorResponse, BrokerError>` accessor returning `ValidatorResponse {
validator: Arc<dyn Validator> }`.

**Why this was wrong, concretely (not just in the abstract)**: grepping
`message-broker-svc` for every call site of `.validator(` and `ValidatorResponse`
turned up exactly four -- the four trait `impl` blocks (`nats`/`kafka`/`postgres`/
`noop`) that were required to implement it. Nothing ever called it: no test, no
factory method, no consumer. Two real bugs were hiding behind it: `NatsMessageBroker::
connect` ran its own ad hoc `url.trim().is_empty()` check instead of calling
`NatsConfig`'s own `Validator::validate`, and `KafkaMessageBroker::new`/
`PostgresMessageBroker::connect` performed no config validation at all before handing
raw strings to their underlying clients. `Validator` was implemented on all three
config types but only ever exercised through `configbuilder`'s TOML-loading path and
unit tests -- never on the actual construction path a real caller uses
(`MessageBrokerFactory::nats/kafka/postgres`).

The root cause: `validator()` was added specifically to be able to say "the config
type maps back to a trait declared in the pattern," in the wake of being pushed to
find a `LedgerPayload`-shaped analog. But `LedgerPayload`'s value comes from being a
generic parameter on the contract's *operational* traits -- the data flowing through
every real call (write/read/verify) is bound by it. Broker config is not that kind of
data: it's consumed once, at construction, and never flows through `publish`/
`subscribe`/`health_check` again -- there's nothing recurring per-call to genericize
`MessageBroker` over. Bolting on an unused accessor method to manufacture a "trait
mapping" was decoration, not the structural mechanic `LedgerPayload` actually provides
-- a `Registry<T: Named>`/`RuntimePhase`-style precedent check would have caught this
the same way it did for `BackendKind`: ask whether the trait's own method signatures
structurally require the addition, not whether it's possible to make some trait
technically apply.

**Decision**: deleted `MessageBroker::validator`, `ValidatorRequest`, and
`ValidatorResponse` outright. `Validator` itself is unaffected -- it remains a real,
minimal self-check trait (`validate(&self, ValidationRequest) -> Result<(),
ValidationError>`), unchanged. `message-broker-svc`'s own amendment wires each
backend's `Validator::validate` impl into its actual constructor
(`NatsMessageBroker::connect`/`KafkaMessageBroker::new`/`PostgresMessageBroker::
connect`), replacing the ad hoc/missing validation with one real call each -- the one
place `Validator` was already correctly load-bearing (`MessageBrokerFactory::
validate<V: Validator>`) needed no change.

**Consequences**: `MessageBroker`'s trait surface shrinks to exactly the three
operations a broker performs (`publish`/`subscribe`/`health_check`); no dead trait
method remains to give a false impression of a `LedgerPayload`-style generic
relationship that this domain doesn't structurally have.

Verified: `cargo build/test` clean (same 9 tests + 1 doctest -- no test depended on
the removed items). `cargo fmt --check` and `cargo clippy --all-targets -- -D
warnings` both clean.

[← Docs index](../../README.md)
