//! [`Validator`] — value validation contract.

use crate::{ValidationError, ValidationRequest};

/// Validates that a value is in a legal state before use.
pub trait Validator: Send + Sync {
    /// Check `self`, returning `Ok(())` when valid, or an actionable
    /// [`ValidationError`] describing the first validation failure.
    /// `request` carries no data of its own — the value being checked is
    /// `self`.
    fn validate(&self, request: ValidationRequest) -> Result<(), ValidationError>;
}
