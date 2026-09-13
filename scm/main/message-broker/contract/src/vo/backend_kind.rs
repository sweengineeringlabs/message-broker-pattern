//! [`BackendKind`] — selects which broker backend a config activates.

/// Which broker backend a `MessageBrokerConfig` activates
/// (`message-broker-pattern-core`).
///
/// Deserialized from the `backend` key of the `[message_broker]` TOML section
/// using snake_case spellings: `"in_memory"`, `"nats"`, `"kafka"`, and `"postgres"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendKind {
    /// In-process broadcast broker. Constructed by `message-broker-svc`.
    InMemory,
    /// NATS server connection. Requires a `url`.
    Nats,
    /// Apache Kafka connection. Requires a `url` (bootstrap brokers, e.g.
    /// `"localhost:9092"`) and a `group_id`.
    Kafka,
    /// Postgres connection using the `pgmq` extension. Requires a `url`
    /// (Postgres DSN) and a `queue_name`.
    Postgres,
}
