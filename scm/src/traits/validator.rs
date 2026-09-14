//! [`Validator`] — value validation contract.

use std::sync::Arc;

use crate::{BrokerError, ValidationError, ValidationRequest, ValidatorResponse};

/// Validates that a value is in a legal state before use.
pub trait Validator: Send + Sync {
    /// Check `self`, returning `Ok(())` when valid, or an actionable
    /// [`ValidationError`] describing the first validation failure.
    /// `request` carries no data of its own — the value being checked is
    /// `self`.
    fn validate(&self, request: ValidationRequest) -> Result<(), ValidationError>;

    /// Run [`Validator::validate`], mapping any violation into a
    /// [`BrokerError::Connection`] so callers can `?` it directly at connect
    /// time. Identical for every implementor — touches only `validate` and
    /// this crate's own types — so it lives here as a default method rather
    /// than being reimplemented, or extended via a separate trait, by every
    /// backend that implements `Validator`.
    fn validate_config(&self) -> Result<(), BrokerError> {
        self.validate(ValidationRequest)
            .map_err(|e| BrokerError::Connection(e.to_string()))
    }

    /// Wrap this shared config handle as a type-erased [`ValidatorResponse`],
    /// for `MessageBroker::validator()` to return. `where Self: Sized`
    /// excludes this from `Validator`'s vtable, so `Arc<dyn Validator>`
    /// (used by [`ValidatorResponse`] itself) stays valid.
    fn validator_response(self: &Arc<Self>) -> ValidatorResponse
    where
        Self: Sized + 'static,
    {
        ValidatorResponse {
            validator: Arc::clone(self) as Arc<dyn Validator>,
        }
    }
}
