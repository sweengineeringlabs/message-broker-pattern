//! [`TaskQueueFactoryContract`] — contract for types that mint task identifiers
//! and construct task handles.

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;

use crate::{QueueError, TaskHandleBuilder, TaskId};

/// Contract for types that mint task identifiers and construct [`TaskHandleBuilder`]s.
///
/// Queue-construction methods (`default_factory`, `build_in_memory`) live as
/// inherent associated functions on each consumer's own factory type — they
/// return implementation-owned concrete types and are not part of this
/// contract.
pub trait TaskQueueFactoryContract {
    /// Generate a fresh [`TaskId`] for use when constructing tasks.
    fn new_task_id(&self) -> TaskId {
        TaskId::new()
    }

    /// Return a [`TaskHandleBuilder`] pre-seeded with the dequeued task's identity.
    ///
    /// Convenience factory so implementors can construct [`TaskHandle`](crate::TaskHandle)
    /// values without depending on the concrete builder type.
    fn build_handle(
        task_id: TaskId,
        payload: Bytes,
        headers: HashMap<String, String>,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> TaskHandleBuilder {
        TaskHandleBuilder::new(task_id, payload, ack, nack).headers(headers)
    }
}
