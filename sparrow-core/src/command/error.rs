// TODO: Add more enum member for better error description
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum CommandError {
  NotFound,
  Malformed,
}

impl std::fmt::Display for CommandError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CommandError::NotFound => write!(f, "Command not found"),
      CommandError::Malformed => write!(f, "Malformed command"),
    }
  }
}
