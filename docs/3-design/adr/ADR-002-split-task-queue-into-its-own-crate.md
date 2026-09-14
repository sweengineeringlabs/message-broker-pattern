# ADR-002: Split `TaskQueue` into its own crate (SRP)

**Status**: Accepted
**Date**: 2026-09-14

## Context

This crate bundled two traits: `MessageBroker` (fan-out/broadcast — every
subscriber gets every message) and `TaskQueue` (competing-consumer — each
task goes to exactly one worker). See "`TaskQueue`'s belated migration" in
`architecture.md` for how `TaskQueue` arrived here — ported from
`edge-runtime`'s superset contract specifically so "a consumer gets this
domain's whole primitive set from one crate," framed at the time as fixing
an incomplete migration, not a deliberate scope decision.

That framing weighed migration-completeness, not single responsibility.
Checked again: `MessageBroker` and `TaskQueue` have different evolution
drivers (pub/sub concerns vs. queue concerns) and different consumers (not
every consumer of one wants the other). Shared arrival history is not
shared responsibility. See
[Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)'s
"Single Responsibility: One Domain Per `-pattern` Crate" section, written
using this exact case study.

## Decision

- New crate: [`task-queue-pattern`](https://github.com/sweengineeringlabs/task-queue-pattern).
  Everything task-specific moves there unchanged: `TaskQueue`,
  `TaskQueueFactoryContract`, `PayloadValidator`,
  `Task`/`TaskHandle`/`TaskHandleBuilder`/`TaskId`, `QueueError`,
  `VALIDATOR_SVC`/`TASK_QUEUE_FACTORY_CONTRACT_ID`/`MAX_TASK_PAYLOAD_BYTES`/
  `TASK_ID_HEADER_KEY`, and the `custom_validator` example.
- This crate keeps `MessageBroker` and its own `Validator` (backend config
  validation — distinct from `task-queue-pattern`'s `PayloadValidator`,
  which is a simpler generic self-check with no request parameter).
- `bytes`/`uuid` dependencies removed — nothing remaining here needs them.
- Breaking change: `0.1.3 → 0.2.0` (pre-1.0 minor bump).

## Consequences

- `task-queue-svc` (companion repo to `task-queue-pattern`) holds every real
  `TaskQueue` implementation, extracted the same way from
  `message-broker-svc` — see that repo's own ADR-001.
- `message-broker-svc`'s own crates lose their `TaskQueue` implementations
  in the same round of work — see that repo's own ADR documenting the
  mirrored split.
- Any consumer wanting both `MessageBroker` and `TaskQueue` now depends on
  two crates instead of one — the correct trade-off per SRP: a consumer
  wanting only one no longer pulls in the other's entire primitive set.
