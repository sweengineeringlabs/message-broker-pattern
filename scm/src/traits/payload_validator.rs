//! [`PayloadValidator`] — validates that a value is in a legal state before use.

/// Validates that a value is in a legal state before use.
///
/// Distinct from [`crate::Validator`]: that trait validates a backend's own
/// config against a [`crate::ValidationRequest`] and returns a structured
/// [`crate::ValidationError`]; this trait is a simpler self-check with no
/// request parameter, used to validate arbitrary values such as a task's
/// payload before it enters a [`crate::TaskQueue`].
pub trait PayloadValidator {
    /// Return `Ok(())` when the value is valid, or `Err` with an actionable
    /// description of the first validation failure.
    fn validate(&self) -> Result<(), String>;
}
