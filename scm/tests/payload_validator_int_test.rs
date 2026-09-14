//! Tests for the [`PayloadValidator`] trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use message_broker_pattern::PayloadValidator;

#[test]
fn test_custom_validator_ok_path() {
    struct AlwaysOk;
    impl PayloadValidator for AlwaysOk {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    let result = AlwaysOk.validate();
    assert_eq!(result, Ok(()), "custom validator must return Ok(())");
}

#[test]
fn test_custom_validator_error_path() {
    struct AlwaysErr;
    impl PayloadValidator for AlwaysErr {
        fn validate(&self) -> Result<(), String> {
            Err("always invalid".into())
        }
    }
    let result = AlwaysErr.validate();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "always invalid");
}
