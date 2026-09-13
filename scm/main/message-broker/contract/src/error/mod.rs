//! Error types — broker error definitions.

pub(crate) mod broker_error;
pub(crate) mod validation_error;

pub use broker_error::BrokerError;
pub use validation_error::ValidationError;
