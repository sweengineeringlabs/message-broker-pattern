//! Tests for the `Validator` trait contract and [`BrokerSvc::validate`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use message_broker_pattern_contract::{ValidationError, ValidationRequest, Validator};
use message_broker_pattern_saf::BrokerSvc;

/// @covers: validate
#[test]
fn test_validate_custom_validator_ok_path_happy() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
            Ok(())
        }
    }
    assert_eq!(AlwaysOk.validate(ValidationRequest), Ok(()));
}

/// @covers: validate
#[test]
fn test_validate_custom_validator_error_path_error() {
    struct AlwaysErr;
    impl Validator for AlwaysErr {
        fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
            Err(ValidationError {
                violations: vec!["always invalid".to_string()],
            })
        }
    }
    let result = AlwaysErr.validate(ValidationRequest);
    assert_eq!(
        result,
        Err(ValidationError {
            violations: vec!["always invalid".to_string()],
        })
    );
}

/// @covers: validate — BrokerSvc::validate delegates to Validator::validate
#[test]
fn test_broker_svc_validate_ok_for_valid_type_happy() {
    struct Valid;
    impl Validator for Valid {
        fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
            Ok(())
        }
    }
    assert_eq!(BrokerSvc::validate(&Valid), Ok(()));
}

/// @covers: validate — BrokerSvc::validate returns err for invalid type
#[test]
fn test_broker_svc_validate_err_for_invalid_type_error() {
    struct Invalid;
    impl Validator for Invalid {
        fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
            Err(ValidationError {
                violations: vec!["bad state".to_string()],
            })
        }
    }
    let result = BrokerSvc::validate(&Invalid);
    assert_eq!(
        result,
        Err(ValidationError {
            violations: vec!["bad state".to_string()],
        })
    );
}

/// @covers: validate — every violation is itemized, not collapsed to the first.
#[test]
fn test_validate_reports_multiple_violations_edge() {
    struct MultiInvalid;
    impl Validator for MultiInvalid {
        fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
            Err(ValidationError {
                violations: vec!["first problem".to_string(), "second problem".to_string()],
            })
        }
    }
    let result = MultiInvalid.validate(ValidationRequest);
    let violations = result.unwrap_err().violations;
    assert_eq!(violations, vec!["first problem", "second problem"]);
}
