//! Trait definitions — public contract.

pub(crate) mod message_broker;
pub(crate) mod validator;

pub use message_broker::MessageBroker;
pub use validator::Validator;
