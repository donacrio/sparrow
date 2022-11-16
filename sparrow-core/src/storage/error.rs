use super::fixed_size::FixedSizeStorageError;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum StorageError {
  FixedSize(FixedSizeStorageError),
}

impl From<FixedSizeStorageError> for StorageError {
  fn from(err: FixedSizeStorageError) -> Self {
    Self::FixedSize(err)
  }
}

impl std::fmt::Display for StorageError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StorageError::FixedSize(err) => write!(f, "FixedSizeStorageError: {}", err),
    }
  }
}
