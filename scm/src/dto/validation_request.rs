//! [`ValidationRequest`] — the marker argument [`crate::Validator::validate`] takes.

/// Marker argument for [`crate::Validator::validate`]. Carries no data — the
/// value being checked is `self`, not a field on this type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValidationRequest;
