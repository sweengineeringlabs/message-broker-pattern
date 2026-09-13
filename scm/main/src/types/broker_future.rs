//! [`BrokerFuture`] — local wrapper around a boxed future, so this crate never
//! names the foreign `futures::future::BoxFuture` type directly in a public
//! signature.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A boxed, pinned, `Send` future — the async return type for
/// [`crate::MessageBroker`] operations.
pub struct BrokerFuture<'a, T>(Pin<Box<dyn Future<Output = T> + Send + 'a>>);

impl<'a, T> BrokerFuture<'a, T> {
    /// Wrap `fut` for use as a trait method's return type.
    pub fn new(fut: impl Future<Output = T> + Send + 'a) -> Self {
        Self(Box::pin(fut))
    }
}

impl<'a, T> Future for BrokerFuture<'a, T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        self.0.as_mut().poll(cx)
    }
}
