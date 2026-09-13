//! [`BrokerSvc`] — message broker public factory surface.
//!
//! All factory functions are implemented as methods on `BrokerSvc`. This
//! crate ships a single no-op reference broker
//! ([`BrokerSvc::noop_broker`]); real backends (NATS, Kafka, Postgres) and
//! the `from_config` construction factory live in `message-broker-svc`.

use configbuilder::ConfigBuilder;
use message_broker_pattern_contract::{
    MessageBroker, ValidationError, ValidationRequest, Validator,
};
use message_broker_pattern_core::{NoopMessageBroker, NoopValidator};

/// Message broker factory and configuration entrypoint.
///
/// Methods on this type are the sole public surface for constructing broker
/// instances and config builders. Consumers call `BrokerSvc::noop_broker()`
/// rather than naming concrete implementation types.
pub struct BrokerSvc;

impl BrokerSvc {
    /// Return a [`ConfigBuilderImpl`](configbuilder::ConfigBuilderImpl) pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> configbuilder::ConfigBuilderImpl {
        configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Construct the no-op reference broker.
    ///
    /// Publishing discards the message and subscribing yields an empty stream.
    /// Intended for tests and as a safe default; production deployments inject a
    /// real backend from `message-broker-svc`.
    pub fn noop_broker() -> Box<dyn MessageBroker> {
        Box::new(NoopMessageBroker)
    }

    /// Validate a value that implements [`Validator`].
    pub fn validate<V: Validator>(v: &V) -> Result<(), ValidationError> {
        v.validate(ValidationRequest)
    }

    /// Construct the no-op reference [`MessageBroker`] (an alias for
    /// [`BrokerSvc::noop_broker`]).
    pub fn create_message_broker() -> Box<dyn MessageBroker> {
        Self::noop_broker()
    }

    /// Construct the always-valid reference [`Validator`], for consumers with
    /// no domain value of their own to validate yet.
    pub fn create_validator() -> Box<dyn Validator> {
        Box::new(NoopValidator)
    }
}
