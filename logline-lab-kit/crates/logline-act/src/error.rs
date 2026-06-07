use thiserror::Error;

/// Errors raised while validating, canonicalizing, or hashing an Act.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ActError {
    #[error("act is missing required slot(s): {0}")]
    MissingSlots(String),
    #[error("act carries non-canonical slot(s): {0}")]
    ExtraSlots(String),
    #[error("value must be a JSON object")]
    NotAnObject,
    #[error("failed to encode JSON string")]
    StringEncoding,
    #[error("invalid act json: {0}")]
    Json(String),
}
