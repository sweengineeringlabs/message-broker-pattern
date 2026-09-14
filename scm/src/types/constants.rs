//! Cross-crate marker/wire-format constants shared by every implementor of
//! this domain's traits.

/// Identifies the validator SAF contract in this crate.
pub const VALIDATOR_SVC: &str = "validator";

/// Identifier for the default task queue factory contract implementation.
pub const TASK_QUEUE_FACTORY_CONTRACT_ID: &str = "task_queue";

/// Maximum task payload size in bytes accepted by the default in-memory queue.
pub const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;

/// Reserved message/task header key carrying the originating [`crate::TaskId`]
/// across backends (Kafka, NATS) whose wire format has no native task-identity
/// field.
///
/// Every backend implementation that needs to preserve task identity across
/// the wire must encode/decode under this exact key — it lives here, not in
/// any one backend crate, so implementations can't silently drift onto
/// different values.
pub const TASK_ID_HEADER_KEY: &str = "x-edge-task-id";
