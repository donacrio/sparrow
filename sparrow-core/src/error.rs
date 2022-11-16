use std::io;

use crate::{command::CommandError, storage::StorageError};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg_attr(test, derive(Debug))]
pub enum Error {
  Storage(StorageError),
  Command(CommandError),
  IO(io::Error),
}

impl From<StorageError> for Error {
  fn from(err: StorageError) -> Self {
    Self::Storage(err)
  }
}

impl From<CommandError> for Error {
  fn from(err: CommandError) -> Self {
    Self::Command(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}
