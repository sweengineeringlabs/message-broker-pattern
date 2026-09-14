//! Integration tests for the saf-facing service-marker constants.

/// @covers: MAX_TASK_PAYLOAD_BYTES
#[test]
fn test_max_task_payload_bytes_is_four_megabytes_happy() {
    assert_eq!(
        message_broker_pattern::MAX_TASK_PAYLOAD_BYTES,
        4 * 1024 * 1024,
        "MAX_TASK_PAYLOAD_BYTES must equal 4 MiB"
    );
}

/// @covers: MAX_TASK_PAYLOAD_BYTES
#[test]
fn test_max_task_payload_bytes_fits_bytes_from_static_edge() {
    let payload = vec![0u8; message_broker_pattern::MAX_TASK_PAYLOAD_BYTES];
    assert_eq!(
        payload.len(),
        message_broker_pattern::MAX_TASK_PAYLOAD_BYTES,
        "a buffer sized to MAX_TASK_PAYLOAD_BYTES must allocate that many bytes"
    );
}
