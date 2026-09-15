# Glossary

Alphabetized list of terms used in `message-broker-pattern`.

---

**MessageBroker** - Cross-process publish/subscribe contract: `publish`/`subscribe`/`health_check`/`validator`. Fans out every published message to every active subscriber of the same topic. `publish`/`subscribe`/`health_check` return `impl Future` (RPITIT), not a boxed future — zero-cost, at the cost of object safety; see `docs/3-design/architecture.md`'s "Why `MessageBroker` returns `impl Future`, not `BrokerFuture`".

**MessageStream** - Type alias for the stream a `MessageBroker::subscribe` call returns (`Pin<Box<dyn Stream<Item = Result<Message, BrokerError>> + Send>>`).

**TaskQueue**, **Task**, **TaskHandle**, **TaskHandleBuilder**, **TaskId**,
**TaskQueueFactoryContract**, **PayloadValidator**, **QueueError** - moved to
[`task-queue-pattern`](https://github.com/sweengineeringlabs/task-queue-pattern)
(SRP — see `docs/3-design/adr/ADR-002`). See that repo's own glossary.

**Validator** - Backend config validation trait (`fn validate(&self, request: ValidationRequest) -> Result<(), ValidationError>`), distinct from `PayloadValidator`. The trait `MessageBroker::validator()` returns a handle to. Also provides `validate_config`/`validator_response` as default methods, built purely from `validate` and this crate's own types — every `Validator` implementor gets them for free.

[← Docs index](README.md)
