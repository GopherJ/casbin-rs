use rhai::{EvalAltResult, ParseError};
use thiserror::Error;

use std::{error::Error as StdError, io::Error as IoError};

/// ModelError represents any type of errors in model configuration
#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Invalid request definition: `{0}`")]
    R(String),
    #[error("Invalid policy definition: `{0}`")]
    P(String),
    #[error("Unsupported effect: `{0}`")]
    E(String),
    #[error("Invalid matcher: `{0}`")]
    M(String),
    #[error("Other: `{0}`")]
    Other(String),
}

/// RequestError represents any type of errors in coming request
#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Request doesn't match request definition. expected length: {0}, found length {1}")]
    UnmatchRequestDefinition(usize, usize),
}

/// PolicyError represents any type of errors in policy
#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("Policy doesn't match policy definition. expected length: {0}, found length {1}")]
    UnmatchPolicyDefinition(usize, usize),
}

/// RBAC error represents any type of errors in RBAC role manager
#[derive(Error, Debug)]
pub enum RbacError {
    #[error("Role `{0}` not found")]
    NotFound(String),
}
/// AdapterError error represents any type of errors in adapter's execution
#[derive(Error, Debug)]
#[error("Adapter error: {0:?}")]
pub struct AdapterError(pub Box<dyn StdError + Send + Sync>);

/// General casbin error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Casbin Io Error: `{0:?}`")]
    IoError(#[from] IoError),

    #[error("Casbin Model Error: `{0:?}`")]
    ModelError(#[from] ModelError),

    #[error("Casbin Policy Error: `{0:?}`")]
    PolicyError(#[from] PolicyError),

    #[error("Casbin Role Manager Error: `{0:?}`")]
    RbacError(#[from] RbacError),

    #[error("Casbin Evaluation Error: `{0:?}`")]
    RhaiError(#[from] Box<EvalAltResult>),
    #[error("Casbin Parse Error: `{0:?}`")]
    RhaiParseError(#[from] ParseError),

    #[error("Casbin Request Error: `{0:?}`")]
    RequestError(#[from] RequestError),

    #[error("Casbin Adapter Error: `{0:?}`")]
    AdapterError(#[from] AdapterError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_send<T: Send>() -> bool {
        true
    }

    fn is_sync<T: Sync>() -> bool {
        true
    }

    #[test]
    fn test_send_sync() {
        assert!(is_send::<Error>());
        assert!(is_sync::<Error>());
    }
}
