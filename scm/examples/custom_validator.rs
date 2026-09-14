//! Shows two things a consumer of this contract crate can do without any
//! implementation dependency: implement `PayloadValidator` against a real
//! rule, and build a `TaskHandle` via the fluent `TaskHandleBuilder`.
use bytes::Bytes;
use futures::future::FutureExt;
use message_broker_pattern::{PayloadValidator, TaskHandleBuilder, TaskId};

struct MaxPayloadSizeValidator {
    payload_bytes: usize,
    limit_bytes: usize,
}

impl PayloadValidator for MaxPayloadSizeValidator {
    fn validate(&self) -> Result<(), String> {
        if self.payload_bytes > self.limit_bytes {
            Err(format!(
                "payload of {} bytes exceeds the {}-byte limit",
                self.payload_bytes, self.limit_bytes
            ))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let payload = Bytes::from_static(b"hello task queue");

    let validator = MaxPayloadSizeValidator {
        payload_bytes: payload.len(),
        limit_bytes: 1024,
    };
    if let Err(e) = validator.validate() {
        eprintln!("payload rejected: {e}");
        return;
    }

    let handle = TaskHandleBuilder::new(
        TaskId::new(),
        payload,
        async { Ok(()) }.boxed(),
        async { Ok(()) }.boxed(),
    )
    .build();

    println!("built task handle for task_id={}", handle.task_id);
}
