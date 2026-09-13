//! Integration tests for [`BrokerSvc::create_config_builder`].

use configbuilder::BuilderFinalizer;
use message_broker_pattern_saf::BrokerSvc;

/// @covers: create_config_builder — returns a pre-seeded builder with this crate's package name
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_name_happy() {
    let loader = BrokerSvc::create_config_builder().build_loader();
    assert!(
        loader.is_ok(),
        "a fresh builder with no config directories must still produce a usable loader"
    );
}

/// @covers: create_config_builder — builder is callable and does not panic
#[test]
fn test_create_config_builder_returns_without_panic_edge() {
    let builder = BrokerSvc::create_config_builder();
    let loader = builder.build_loader();
    assert!(loader.is_ok());
}
