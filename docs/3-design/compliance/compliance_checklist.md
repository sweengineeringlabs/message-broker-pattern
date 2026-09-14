# Architecture Compliance Checklist

**Audience**: Architects, contributors, reviewers.

Derived from [architecture.md](../architecture.md). Every rule here is enforceable —
re-run the listed command after any change and expect the stated result.

## 1. Zero implementation

| # | Rule | Verify |
|---|------|--------|
| 1 | This crate implements none of its own primary traits (`MessageBroker`, `Validator`) | `grep -rn "^impl MessageBroker for\|^impl Validator for" scm/src/` returns nothing |
| 2 | No no-op or reference implementation lives here | Same as above — `message-broker-svc-saf` owns `NoopMessageBroker` |

## 2. No backend-technology vocabulary

| # | Rule | Verify |
|---|------|--------|
| 3 | No type/field/enum-variant names a specific backend technology (no `BackendKind`-shaped enum, no field named after one backend's own concept like `group_id`/`queue_name`) | `grep -rnE "^\s*(pub )?(struct\|enum\|fn) \w*(Kafka\|Nats\|Postgres\|Redis)" scm/src/` returns nothing. Doc-comment prose describing that *other* crates implement these backends is expected and not a violation — see `lib.rs`'s own module doc for an example |
| 4 | No shared config/backend-selector type of any kind | `grep -rn "BackendKind\|MessageBrokerConfig" scm/src/` returns nothing |

## 3. No application-composition vocabulary

| # | Rule | Verify |
|---|------|--------|
| 5 | No config-section name, default-backend value, or `ApplicationConfig`-shaped type | `grep -rn "ApplicationConfig\|BrokerBackendConfig\|ConfigProvider" scm/src/` returns nothing |

## 4. Single responsibility — `MessageBroker` only

| # | Rule | Verify |
|---|------|--------|
| 6 | `TaskQueue` and its own primitive set (`Task`/`TaskHandle`/`TaskHandleBuilder`/`TaskId`/`QueueError`/`PayloadValidator`/`TaskQueueFactoryContract`) do not live here — they belong to `task-queue-pattern` (SRP, see ADR-002) | `grep -rn "TaskQueue\|PayloadValidator\|QueueError\|TaskHandle\|TaskId\b" scm/src/` returns nothing outside doc-comment prose pointing to `task-queue-pattern` |
| 7 | No primitive that belongs to a different responsibility is added here "for consumer convenience" without a fresh SRP check against [Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)'s own test | Manual review on any future addition |

## 5. Dependency footprint

| # | Rule | Verify |
|---|------|--------|
| 8 | Dependency footprint stays exactly `futures` + `thiserror` | `cargo tree --depth 1` after any `Cargo.toml` change — anything else is a regression |

## 6. Lint gates

| # | Rule | Verify |
|---|------|--------|
| 9 | `#![deny(unsafe_code)]` enforced | `cargo build` fails on any `unsafe` block |
| 10 | `#![warn(missing_docs)]` enforced | `cargo doc --no-deps` warns on any undocumented public item |
| 11 | `cargo clippy --all-targets -- -D warnings` clean | Run before every commit |
| 12 | `cargo fmt --check` clean | Run before every commit |

[← 3-design index](../README.md)
