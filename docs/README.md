# Documentation Index

**Audience**: All — architects, contributors, and consumers evaluating this crate.

Docs for [`message-broker-pattern`](https://github.com/sweengineeringlabs/message-broker-pattern),
this domain's whole primitive set — `MessageBroker`, `TaskQueue`, `Validator`,
`PayloadValidator`, and their value/error/DTO types — zero implementation, zero backend
knowledge.

| Section | What's there |
|---------|--------------|
| [0-ideation/papers](0-ideation/papers/README.md) | Prior-art papers, currently empty |
| [3-design](3-design/README.md) | Architecture, compliance checklist, and ADRs: why this crate is a single flat package, why `TaskQueue` lives here too, why no backend vocabulary is ever allowed here |
| [4-development](4-development/README.md) | Developer guide: repo structure, branching, testing conventions |
| [glossary.md](glossary.md) | Domain terminology |

New to this repo? Start with [3-design/architecture.md](3-design/architecture.md).
