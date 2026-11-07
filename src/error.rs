use crate::id::ObjectId;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("object not found: {0}")]
  NotFound(ObjectId),
  #[error("unexpected kind for {id}: expected {expected}, got {actual}")]
  KindMismatch { id: ObjectId, expected: &'static str, actual: &'static str },
  #[error("hash mismatch: expected {expected}, got {actual}")]
  HashMismatch { expected: ObjectId, actual: ObjectId },
  #[error("parse error: {0}")]
  Parse(String),
  #[error("invalid: {0}")]
  Invalid(String),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
