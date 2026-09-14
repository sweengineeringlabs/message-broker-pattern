//! Integration tests for the [`TaskQueue`] trait contract (rule 222).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::redundant_pattern_matching
)]

use futures::future::BoxFuture;
use message_broker_pattern::{QueueError, Task, TaskHandle, TaskQueue};

struct AlwaysOkQueue;
struct AlwaysErrQueue;

impl TaskQueue for AlwaysOkQueue {
    fn enqueue(&self, _task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Ok(()) })
    }
    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        Box::pin(async { Ok(None) })
    }
    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Ok(()) })
    }
}

impl TaskQueue for AlwaysErrQueue {
    fn enqueue(&self, _task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Err(QueueError::Connection("stub queue unavailable".into())) })
    }
    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        Box::pin(async { Err(QueueError::Connection("stub queue unavailable".into())) })
    }
    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Err(QueueError::Connection("stub queue unavailable".into())) })
    }
}

// ── TaskQueue::enqueue (rule 222) ────────────────────────────────────────────

/// @covers: TaskQueue::enqueue
#[test]
fn test_enqueue_task_into_ok_queue_happy() {
    let queue = AlwaysOkQueue;
    let task = Task::new(b"payload".as_ref());
    let result = futures::executor::block_on(queue.enqueue(task));
    assert!(matches!(result, Ok(())), "enqueue must return Ok(())");
}

/// @covers: TaskQueue::enqueue
#[test]
fn test_enqueue_task_into_failing_queue_error() {
    let queue = AlwaysErrQueue;
    let task = Task::new(b"payload".as_ref());
    assert!(matches!(
        futures::executor::block_on(queue.enqueue(task)),
        Err(QueueError::Connection(_))
    ));
}

/// @covers: TaskQueue::enqueue
#[test]
fn test_enqueue_empty_payload_task_edge() {
    let queue = AlwaysOkQueue;
    let task = Task::new(b"".as_ref());
    let result = futures::executor::block_on(queue.enqueue(task));
    assert!(
        matches!(result, Ok(())),
        "enqueue with empty payload must return Ok(())"
    );
}

// ── TaskQueue::dequeue (rule 222) ────────────────────────────────────────────

/// @covers: TaskQueue::dequeue
#[test]
fn test_dequeue_from_empty_ok_queue_happy() {
    let queue = AlwaysOkQueue;
    assert!(matches!(
        futures::executor::block_on(queue.dequeue()),
        Ok(None)
    ));
}

/// @covers: TaskQueue::dequeue
#[test]
fn test_dequeue_from_failing_queue_error() {
    let queue = AlwaysErrQueue;
    assert!(matches!(
        futures::executor::block_on(queue.dequeue()),
        Err(QueueError::Connection(_))
    ));
}

/// @covers: TaskQueue::dequeue
#[test]
fn test_dequeue_returns_option_type_edge() {
    let queue = AlwaysOkQueue;
    // Edge: dequeue returns Option — None means empty, not an error.
    let result: Result<Option<TaskHandle>, QueueError> =
        futures::executor::block_on(queue.dequeue());
    assert!(result.is_ok(), "empty dequeue must return Ok(None)");
    let inner = result.unwrap();
    assert!(inner.is_none(), "empty queue must return None");
}

// ── TaskQueue::health_check (rule 222) ───────────────────────────────────────

/// @covers: TaskQueue::health_check
#[test]
fn test_health_check_on_ok_queue_happy() {
    let queue = AlwaysOkQueue;
    let health = futures::executor::block_on(queue.health_check());
    assert!(matches!(health, Ok(())), "health check must return Ok(())");
}

/// @covers: TaskQueue::health_check
#[test]
fn test_health_check_on_failing_queue_error() {
    let queue = AlwaysErrQueue;
    assert!(matches!(
        futures::executor::block_on(queue.health_check()),
        Err(QueueError::Connection(_))
    ));
}

/// @covers: TaskQueue::health_check
#[test]
fn test_health_check_is_idempotent_edge() {
    let queue = AlwaysOkQueue;
    let check1 = futures::executor::block_on(queue.health_check());
    let check2 = futures::executor::block_on(queue.health_check());
    assert!(
        matches!(check1, Ok(())),
        "first health check must return Ok(())"
    );
    assert!(
        matches!(check2, Ok(())),
        "second health check must return Ok(())"
    );
}
