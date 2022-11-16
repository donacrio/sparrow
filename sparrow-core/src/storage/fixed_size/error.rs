#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum FixedSizeStorageError {
  Full,
}

impl std::fmt::Display for FixedSizeStorageError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FixedSizeStorageError::Full => write!(f, "Storage is full"),
    }
  }
}
