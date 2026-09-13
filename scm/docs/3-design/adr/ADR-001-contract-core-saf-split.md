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

[← Docs index](../../README.md)
