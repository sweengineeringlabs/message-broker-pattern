//! Integration tests for the backend-owned `[message_broker]` section.
//!
//! Exercises `MessageBrokerConfig` as an `OptionalSection`: presence-based
//! enabling, the `enabled = false` disable toggle, `deny_unknown_fields`
//! strictness, and cross-field validation.
//!
//! Construction from a config directory (`from_config`) lives in
//! `message-broker-svc`; this crate only owns the config vocabulary.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::{ConfigError, ConfigLoaderFactory, FeatureStateOps, OptionalSection};
use message_broker_pattern_contract::BackendKind;
use message_broker_pattern_core::MessageBrokerConfig;
use tempfile::TempDir;

/// Write `content` to `application.toml` in a fresh temp dir and return a loader
/// rooted at that dir, along with the dir guard (kept alive by the caller).
fn loader_with(content: &str) -> (TempDir, configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

// ── canonical contract ──────────────────────────────────────────────────────

/// @covers: section_name — the canonical TOML key is owned by this crate.
#[test]
fn test_section_name_is_message_broker() {
    assert_eq!(MessageBrokerConfig::section_name(), "message_broker");
}

/// @covers: metadata — backend annotates the feature for startup summaries.
#[test]
fn test_metadata_describes_feature_and_owner() {
    let meta = MessageBrokerConfig::metadata();
    assert!(
        !meta.description.is_empty(),
        "metadata must carry a human-readable description"
    );
    assert_eq!(meta.owner, "platform-team");
    assert_eq!(meta.deprecated_since, None);
}

// ── presence-based enabling ───────────────────────────────────────────────────

/// @covers: load_optional — an absent section resolves to Disabled, not an error.
#[test]
fn test_load_absent_section_returns_disabled() {
    let (_dir, loader) = loader_with("[unrelated]\nkey = \"value\"");
    let state =
        MessageBrokerConfig::load_optional(&loader).expect("absent section is not an error");
    assert!(state.is_disabled());
}

/// @covers: load_optional — presence of the section enables it; fields parse.
#[test]
fn test_load_in_memory_present_returns_enabled() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"");
    let state = MessageBrokerConfig::load_optional(&loader).expect("valid section loads");
    let cfg = state.into_option().expect("section present => Enabled");
    assert_eq!(cfg.backend, BackendKind::InMemory);
    assert_eq!(cfg.url, None);
}

/// @covers: load_optional — nats backend with a url parses and validates.
#[test]
fn test_load_nats_with_url_returns_enabled() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"nats\"\nurl = \"nats://nats.internal:4222\"");
    let state = MessageBrokerConfig::load_optional(&loader).expect("valid nats section loads");
    let cfg = state.into_option().expect("section present => Enabled");
    assert_eq!(cfg.backend, BackendKind::Nats);
    assert_eq!(cfg.url.as_deref(), Some("nats://nats.internal:4222"));
}

// ── disable toggle ────────────────────────────────────────────────────────────

/// @covers: enabled = false — disables a section that is otherwise present,
/// and is accepted despite `deny_unknown_fields` (loader short-circuits first).
#[test]
fn test_enabled_false_disables_present_section() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"\nenabled = false");
    let state = MessageBrokerConfig::load_optional(&loader).expect("enabled=false is not an error");
    assert!(
        state.is_disabled(),
        "enabled = false must disable a present section"
    );
}

// ── deny_unknown_fields strictness ────────────────────────────────────────────

/// @covers: deny_unknown_fields — `enabled = true` is rejected as an unknown
/// field. This is the ADR-006 gotcha: presence enables, so `enabled = true` is
/// both redundant and (under deny_unknown_fields) a parse error.
#[test]
fn test_enabled_true_is_rejected_by_deny_unknown_fields() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"\nenabled = true");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("enabled = true must be rejected by deny_unknown_fields");
    assert!(
        matches!(err, ConfigError::Parse(_)),
        "expected a Parse error for the unknown `enabled` key, got {err:?}"
    );
}

/// @covers: deny_unknown_fields — an arbitrary unknown key is rejected.
#[test]
fn test_unknown_field_is_rejected() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"\nbogus = 1");
    let err =
        MessageBrokerConfig::load_optional(&loader).expect_err("unknown field must be rejected");
    assert!(
        matches!(err, ConfigError::Parse(_)),
        "expected a Parse error for the unknown `bogus` key, got {err:?}"
    );
}

// ── cross-field validation ────────────────────────────────────────────────────

/// @covers: validate_enabled — nats without a url is a validation error.
#[test]
fn test_nats_without_url_returns_validation_error() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"nats\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("nats without url must fail validation");
    assert!(
        matches!(err, ConfigError::Validation { .. }),
        "expected Validation error, got {err:?}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains("url"),
        "error must name the offending field: {msg}"
    );
    assert!(
        msg.contains("message_broker"),
        "error must name the section: {msg}"
    );
}

/// @covers: validate_enabled — nats with an empty url is rejected (not just None).
#[test]
fn test_nats_with_blank_url_returns_validation_error() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"nats\"\nurl = \"   \"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("nats with blank url must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
}

/// @covers: validate_enabled — in_memory with a url is a misconfiguration.
#[test]
fn test_in_memory_with_url_returns_validation_error() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"in_memory\"\nurl = \"nats://x:4222\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("in_memory with url must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
}

/// @covers: validate_enabled — kafka requires both url and group_id.
#[test]
fn test_kafka_without_url_returns_validation_error() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"kafka\"\ngroup_id = \"workers\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("kafka without url must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
    assert!(
        err.to_string().contains("url"),
        "error must mention `url`: {err}"
    );
}

/// @covers: validate_enabled — kafka without group_id is rejected.
#[test]
fn test_kafka_without_group_id_returns_validation_error() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"kafka\"\nurl = \"broker:9092\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("kafka without group_id must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
    assert!(
        err.to_string().contains("group_id"),
        "error must mention `group_id`: {err}"
    );
}

/// @covers: load_optional — kafka with url + group_id enables and parses.
#[test]
fn test_kafka_with_url_and_group_id_returns_enabled() {
    let (_dir, loader) = loader_with(
        "[message_broker]\nbackend = \"kafka\"\nurl = \"broker:9092\"\ngroup_id = \"workers\"",
    );
    let state = MessageBrokerConfig::load_optional(&loader).expect("valid kafka section loads");
    let cfg = state.into_option().expect("section present => Enabled");
    assert_eq!(cfg.backend, BackendKind::Kafka);
    assert_eq!(cfg.url.as_deref(), Some("broker:9092"));
    assert_eq!(cfg.group_id.as_deref(), Some("workers"));
}

/// @covers: backend deserialization — an unknown backend value is rejected.
#[test]
fn test_unknown_backend_value_is_rejected() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"rabbitmq\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("unknown backend variant must fail to parse");
    assert!(matches!(err, ConfigError::Parse(_)), "got {err:?}");
}

/// @covers: validate_enabled — postgres requires both url and queue_name.
#[test]
fn test_postgres_without_url_returns_validation_error() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"postgres\"\nqueue_name = \"edge_events\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("postgres without url must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
    assert!(
        err.to_string().contains("url"),
        "error must mention `url`: {err}"
    );
}

/// @covers: validate_enabled — postgres without queue_name is rejected.
#[test]
fn test_postgres_without_queue_name_returns_validation_error() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"postgres\"\nurl = \"postgres://localhost/app\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("postgres without queue_name must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
    assert!(
        err.to_string().contains("queue_name"),
        "error must mention `queue_name`: {err}"
    );
}

/// @covers: validate_enabled — postgres does not accept a group_id.
#[test]
fn test_postgres_with_group_id_returns_validation_error() {
    let (_dir, loader) = loader_with(
        "[message_broker]\nbackend = \"postgres\"\nurl = \"postgres://localhost/app\"\n\
         queue_name = \"edge_events\"\ngroup_id = \"workers\"",
    );
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("postgres with group_id must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
    assert!(
        err.to_string().contains("group_id"),
        "error must mention `group_id`: {err}"
    );
}

/// @covers: load_optional — postgres with url + queue_name enables and parses.
#[test]
fn test_postgres_with_url_and_queue_name_returns_enabled() {
    let (_dir, loader) = loader_with(
        "[message_broker]\nbackend = \"postgres\"\nurl = \"postgres://localhost/app\"\n\
         queue_name = \"edge_events\"",
    );
    let state = MessageBrokerConfig::load_optional(&loader).expect("valid postgres section loads");
    let cfg = state.into_option().expect("section present => Enabled");
    assert_eq!(cfg.backend, BackendKind::Postgres);
    assert_eq!(cfg.url.as_deref(), Some("postgres://localhost/app"));
    assert_eq!(cfg.queue_name.as_deref(), Some("edge_events"));
}

/// @covers: validate_enabled — non-postgres backends reject a queue_name.
#[test]
fn test_kafka_with_queue_name_returns_validation_error() {
    let (_dir, loader) = loader_with(
        "[message_broker]\nbackend = \"kafka\"\nurl = \"broker:9092\"\n\
         group_id = \"workers\"\nqueue_name = \"edge_events\"",
    );
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("kafka with queue_name must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
    assert!(
        err.to_string().contains("queue_name"),
        "error must mention `queue_name`: {err}"
    );
}
