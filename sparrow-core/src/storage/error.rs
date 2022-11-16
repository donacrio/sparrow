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
