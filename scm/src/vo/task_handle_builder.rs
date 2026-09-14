//! [`TaskHandleBuilder`] — fluent constructor for [`TaskHandle`].

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;

use crate::{QueueError, TaskHandle, TaskId};

/// Fluent builder for [`TaskHandle`].
///
/// All required fields (`task_id`, `payload`, `ack`, `nack`) are supplied at
/// construction time; optional fields (`headers`) use fluent setters.
/// Call [`build`](TaskHandleBuilder::build) to produce the [`TaskHandle`].
pub struct TaskHandleBuilder {
    pub(crate) task_id: TaskId,
    pub(crate) payload: Bytes,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) ack: BoxFuture<'static, Result<(), QueueError>>,
    pub(crate) nack: BoxFuture<'static, Result<(), QueueError>>,
}

impl TaskHandleBuilder {
    /// Create a builder with all required fields.
    pub fn new(
        task_id: TaskId,
        payload: Bytes,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        Self {
            task_id,
            payload,
            headers: HashMap::new(),
            ack,
            nack,
        }
    }

    /// Set the optional key-value metadata headers for the task handle.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Consume the builder and produce a [`TaskHandle`].
    pub fn build(self) -> TaskHandle {
        TaskHandle::new(
            self.task_id,
            self.payload,
            self.headers,
            self.ack,
            self.nack,
        )
    }
}
