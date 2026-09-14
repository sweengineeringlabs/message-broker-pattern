# Glossary

Alphabetized list of terms used in `message-broker-pattern`.

---

**BrokerFuture** - Boxed, pinned future type every `MessageBroker` method returns (`Pin<Box<dyn Future<Output = T> + Send>>`). Needs only `std::future::Future`.

**MessageBroker** - Cross-process publish/subscribe contract: `publish`/`subscribe`/`health_check`/`validator`. Fans out every published message to every active subscriber of the same topic.

**MessageStream** - Type alias for the stream a `MessageBroker::subscribe` call returns (`Pin<Box<dyn Stream<Item = Result<Message, BrokerError>> + Send>>`).

**PayloadValidator** - Generic self-validation trait (`fn validate(&self) -> Result<(), String>`), distinct from `Validator`. Used to validate arbitrary values, such as a task's payload before it enters a `TaskQueue`.

**Task** - The unit of exchange for `TaskQueue`: a payload plus optional metadata headers, addressed by a unique `TaskId`.

**TaskHandle** - Returned by `TaskQueue::dequeue`. Carries the dequeued task's payload/headers plus `ack`/`nack` futures the consumer must call exactly one of.

**TaskHandleBuilder** - Fluent builder for `TaskHandle`, used by implementors that construct one without depending on its concrete fields directly.

**TaskId** - Unique identifier for a `Task`, wrapping a `Uuid`.

**TaskQueue** - Competing-consumer work queue contract: `enqueue`/`dequeue`/`health_check`. Unlike `MessageBroker`, delivers each message to exactly one competing consumer, not every subscriber.

**TaskQueueFactoryContract** - Contract for types that mint fresh `TaskId`s and construct `TaskHandleBuilder`s (`new_task_id`/`build_handle`).

**Validator** - Backend config validation trait (`fn validate(&self, request: ValidationRequest) -> Result<(), ValidationError>`), distinct from `PayloadValidator`. The trait `MessageBroker::validator()` returns a handle to.

[← Docs index](README.md)
