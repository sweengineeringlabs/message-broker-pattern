//! Integration tests for the saf-facing service-marker constants.

/// @covers: TASK_QUEUE_FACTORY_CONTRACT_ID
#[test]
fn test_task_queue_factory_contract_id_is_task_queue_happy() {
    assert_eq!(
        message_broker_pattern::TASK_QUEUE_FACTORY_CONTRACT_ID,
        "task_queue",
        "TASK_QUEUE_FACTORY_CONTRACT_ID must equal \"task_queue\""
    );
}

/// @covers: TASK_QUEUE_FACTORY_CONTRACT_ID
#[test]
fn test_task_queue_factory_contract_id_has_no_whitespace_edge() {
    assert!(
        !message_broker_pattern::TASK_QUEUE_FACTORY_CONTRACT_ID.contains(char::is_whitespace),
        "TASK_QUEUE_FACTORY_CONTRACT_ID must not contain whitespace"
    );
}
