//! Request for [`crate::MessageBroker::health_check`].

/// Request for [`crate::MessageBroker::health_check`]. Carries no data — the
/// operation probes connectivity, it does not take parameters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HealthCheckRequest;
