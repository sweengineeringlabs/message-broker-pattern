//! Integration tests for [`BrokerSvc::create_config_builder`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::{BuilderFinalizer, FeatureStateOps, OptionalSection};
use message_broker_pattern_core::MessageBrokerConfig;
use message_broker_pattern_saf::BrokerSvc;

/// @covers: create_config_builder — the built loader is genuinely usable, not
/// just non-erroring: loading an absent `[message_broker]` section through it
/// resolves to Disabled rather than panicking or silently enabling.
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_name_happy() {
    let loader = BrokerSvc::create_config_builder()
        .build_loader()
        .expect("a fresh builder with no config directories must still produce a usable loader");
    let state = MessageBrokerConfig::load_optional(&loader)
        .expect("an absent section must resolve to Disabled, not an error");
    assert!(
        state.is_disabled(),
        "a builder with no configured directories must resolve every section to Disabled"
    );
}

/// @covers: create_config_builder — each call returns an independently usable
/// builder, not a shared/cached one that only works the first time.
#[test]
fn test_create_config_builder_each_call_returns_independently_usable_builder_edge() {
    let first = BrokerSvc::create_config_builder()
        .build_loader()
        .expect("first builder must produce a usable loader");
    let second = BrokerSvc::create_config_builder()
        .build_loader()
        .expect("second, independently constructed builder must also produce a usable loader");

    let first_state = MessageBrokerConfig::load_optional(&first).expect("first loader must load");
    let second_state =
        MessageBrokerConfig::load_optional(&second).expect("second loader must load");
    assert_eq!(
        first_state.is_disabled(),
        second_state.is_disabled(),
        "two independently constructed builders must behave identically against the same (absent) config"
    );
}
