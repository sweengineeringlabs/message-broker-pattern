//! Value objects — plain data, no domain-trait implementations.

mod backend_kind;
#[allow(clippy::module_inception)]
mod message;

pub use backend_kind::BackendKind;
pub use message::Message;
