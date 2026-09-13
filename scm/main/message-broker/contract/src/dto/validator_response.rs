//! Response for [`crate::MessageBroker::validator`].

use std::sync::Arc;

use crate::Validator;

/// Response for [`crate::MessageBroker::validator`] — a handle to the
/// broker's own config validator, so a caller can revalidate a live broker's
/// configuration (e.g. for health dashboards or hot-reload checks) without
/// naming the concrete config type.
pub struct ValidatorResponse {
    /// The broker's own config validator.
    pub validator: Arc<dyn Validator>,
}
