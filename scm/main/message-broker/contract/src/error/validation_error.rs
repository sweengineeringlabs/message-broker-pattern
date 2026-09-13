//! [`ValidationError`] — the error type for [`crate::Validator::validate`].

/// Errors returned by [`crate::Validator::validate`].
///
/// Wraps every violation found, not just the first — callers can report the
/// full set of problems in one pass instead of fixing them one at a time.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{}", violations.join("; "))]
pub struct ValidationError {
    /// Every validation failure found, in the order they were checked.
    pub violations: Vec<String>,
}
