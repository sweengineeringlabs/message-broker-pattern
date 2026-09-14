//! Value objects — plain data, no domain-trait implementations.

#[allow(clippy::module_inception)]
mod message;
mod task;
mod task_handle;
mod task_handle_builder;
mod task_id;

pub use message::Message;
pub use task::Task;
pub use task_handle::TaskHandle;
pub use task_handle_builder::TaskHandleBuilder;
pub use task_id::TaskId;
