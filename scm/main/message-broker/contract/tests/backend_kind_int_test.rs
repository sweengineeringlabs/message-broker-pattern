//! Integration tests for [`BackendKind`] deserialization.
//!
//! `BackendKind` is the typed selector for the `[message_broker].backend` key.
//! These tests pin its snake_case wire spellings and prove unknown/miscased
//! values are rejected rather than silently defaulted.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use message_broker_pattern_contract::BackendKind;

#[derive(serde::Deserialize)]
struct Holder {
    backend: BackendKind,
}

fn parse(s: &str) -> Result<Holder, toml::de::Error> {
    toml::from_str(s)
}

/// @covers: BackendKind — `"in_memory"` deserializes to the in-memory variant.
#[test]
fn test_backend_kind_in_memory_deserializes() {
    let h = parse("backend = \"in_memory\"").expect("in_memory parses");
    assert_eq!(h.backend, BackendKind::InMemory);
}

/// @covers: BackendKind — `"nats"` deserializes to the nats variant.
#[test]
fn test_backend_kind_nats_deserializes() {
    let h = parse("backend = \"nats\"").expect("nats parses");
    assert_eq!(h.backend, BackendKind::Nats);
}

/// @covers: BackendKind — `"kafka"` deserializes to the kafka variant.
#[test]
fn test_backend_kind_kafka_deserializes() {
    let h = parse("backend = \"kafka\"").expect("kafka parses");
    assert_eq!(h.backend, BackendKind::Kafka);
}

/// @covers: BackendKind — `"postgres"` deserializes to the postgres variant.
#[test]
fn test_backend_kind_postgres_deserializes() {
    let h = parse("backend = \"postgres\"").expect("postgres parses");
    assert_eq!(h.backend, BackendKind::Postgres);
}

/// @covers: BackendKind — an unknown spelling is rejected, not silently defaulted.
#[test]
fn test_backend_kind_unknown_value_is_rejected() {
    assert!(
        parse("backend = \"rabbitmq\"").is_err(),
        "unknown backend value must not parse"
    );
}

/// @covers: BackendKind — only snake_case is accepted (PascalCase is rejected).
#[test]
fn test_backend_kind_rejects_non_snake_case() {
    assert!(
        parse("backend = \"InMemory\"").is_err(),
        "rename_all = snake_case must reject PascalCase spellings"
    );
}
