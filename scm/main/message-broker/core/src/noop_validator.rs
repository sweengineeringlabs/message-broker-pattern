//! [`NoopValidator`] — always-valid reference `Validator`, for backends (like
//! [`crate::NoopMessageBroker`]) with no config of their own to validate.

use message_broker_pattern_contract::{ValidationError, ValidationRequest, Validator};

/// Always-valid reference [`Validator`].
pub struct NoopValidator;

impl Validator for NoopValidator {
    fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
        Ok(())
    }
}
