//! [`MessageBrokerConfig`] — backend-owned `[message_broker]` TOML contract,
//! plus its [`Validator`] and `OptionalSection` (configbuilder
//! auto-configuration) implementations.
//!
//! Lives in `core/`, not `contract/`: the struct declaration carries real
//! cross-field validation logic and an external `configbuilder` dependency,
//! not a pure zero-implementation contract shape.

use configbuilder::{ConfigError, FeatureMetadata, OptionalSection};
use message_broker_pattern_contract::{BackendKind, ValidationError, ValidationRequest, Validator};

/// Canonical configuration for the `[message_broker]` TOML section.
///
/// This struct is owned by the message-broker pattern: the crate that
/// implements the broker also defines the section name, field shape, and
/// validation rules. Consumers opt in by adding this crate to `Cargo.toml`
/// and a `[message_broker]` section to `application.toml` — they never
/// redefine the struct.
///
/// # Enabling
///
/// The feature is enabled by the **presence** of the `[message_broker]`
/// section. To disable a section that is otherwise present, set
/// `enabled = false`. Do **not** write `enabled = true`: it is redundant, and
/// because this struct uses `#[serde(deny_unknown_fields)]` the loader would
/// reject `enabled` as an unknown field. (`enabled = false` is safe — the
/// loader interprets it and short-circuits before deserialization.)
///
/// # Examples
///
/// NATS:
/// ```toml
/// [message_broker]
/// backend = "nats"
/// url     = "nats://nats.internal:4222"
/// ```
///
/// Kafka:
/// ```toml
/// [message_broker]
/// backend  = "kafka"
/// url      = "kafka-broker-1:9092,kafka-broker-2:9092"
/// group_id = "my-service"
/// ```
///
/// Postgres (pgmq):
/// ```toml
/// [message_broker]
/// backend    = "postgres"
/// url        = "postgres://user:pass@localhost/app"
/// queue_name = "edge_events"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageBrokerConfig {
    /// Which backend to construct.
    pub backend: BackendKind,

    /// Server URL for network backends.
    ///
    /// - `nats`: NATS server URL (e.g. `"nats://host:4222"`). Required.
    /// - `kafka`: Comma-separated bootstrap brokers (e.g. `"broker1:9092,broker2:9092"`). Required.
    /// - `postgres`: Postgres DSN (e.g. `"postgres://user:pass@host/db"`). Required.
    /// - `in_memory`: Must be absent.
    #[serde(default)]
    pub url: Option<String>,

    /// Consumer group identifier. Required when `backend = "kafka"`; must be absent otherwise.
    #[serde(default)]
    pub group_id: Option<String>,

    /// `pgmq` queue name. Required when `backend = "postgres"`; must be absent otherwise.
    #[serde(default)]
    pub queue_name: Option<String>,
}

impl MessageBrokerConfig {
    fn validate_fields(&self) -> Result<(), String> {
        match self.backend {
            BackendKind::Nats => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err("backend = \"nats\" requires a non-empty `url` \
                         (e.g. url = \"nats://host:4222\")"
                        .to_string());
                }
                if self.group_id.is_some() {
                    return Err(
                        "backend = \"nats\" does not accept a `group_id`; remove it".to_string()
                    );
                }
                if self.queue_name.is_some() {
                    return Err(
                        "backend = \"nats\" does not accept a `queue_name`; remove it".to_string(),
                    );
                }
            }
            BackendKind::InMemory => {
                if self.url.is_some() {
                    return Err("backend = \"in_memory\" does not accept a `url`; \
                         remove it or set backend = \"nats\", \"kafka\", or \"postgres\""
                        .to_string());
                }
                if self.group_id.is_some() {
                    return Err(
                        "backend = \"in_memory\" does not accept a `group_id`; remove it"
                            .to_string(),
                    );
                }
                if self.queue_name.is_some() {
                    return Err(
                        "backend = \"in_memory\" does not accept a `queue_name`; remove it"
                            .to_string(),
                    );
                }
            }
            BackendKind::Kafka => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err("backend = \"kafka\" requires a non-empty `url` \
                         (bootstrap brokers, e.g. url = \"broker1:9092,broker2:9092\")"
                        .to_string());
                }
                let group_set = self
                    .group_id
                    .as_deref()
                    .is_some_and(|g| !g.trim().is_empty());
                if !group_set {
                    return Err("backend = \"kafka\" requires a non-empty `group_id`".to_string());
                }
                if self.queue_name.is_some() {
                    return Err(
                        "backend = \"kafka\" does not accept a `queue_name`; remove it".to_string(),
                    );
                }
            }
            BackendKind::Postgres => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err("backend = \"postgres\" requires a non-empty `url` \
                         (Postgres DSN, e.g. url = \"postgres://user:pass@host/db\")"
                        .to_string());
                }
                let queue_set = self
                    .queue_name
                    .as_deref()
                    .is_some_and(|q| !q.trim().is_empty());
                if !queue_set {
                    return Err(
                        "backend = \"postgres\" requires a non-empty `queue_name`".to_string()
                    );
                }
                if self.group_id.is_some() {
                    return Err(
                        "backend = \"postgres\" does not accept a `group_id`; remove it"
                            .to_string(),
                    );
                }
            }
        }
        Ok(())
    }
}

impl Validator for MessageBrokerConfig {
    fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
        self.validate_fields().map_err(|reason| ValidationError {
            violations: vec![reason],
        })
    }
}

impl OptionalSection for MessageBrokerConfig {
    // @allow: no_stub_fn_bodies — returns the canonical section key, not a stub
    fn section_name() -> &'static str {
        "message_broker"
    }

    fn validate_enabled(&self) -> Result<(), ConfigError> {
        self.validate_fields()
            .map_err(|reason| ConfigError::Validation {
                section: Self::section_name().to_string(),
                reason,
            })
    }

    fn metadata() -> FeatureMetadata {
        FeatureMetadata {
            description: "cross-process pub/sub message broker",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}
