//! Request for [`crate::MessageBroker::validator`].

/// Request for [`crate::MessageBroker::validator`]. Carries no data — the
/// operation only returns a handle to the broker's own config validator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValidatorRequest;
