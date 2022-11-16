// TODO: Add more enum member for better error description
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum CommandError {
  NotFound(String),
  Malformed(String),
}

impl std::fmt::Display for CommandError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CommandError::NotFound(cmd) => write!(f, "Command not found: {}", cmd),
      CommandError::Malformed(expl) => write!(f, "Malformed command: {}", expl),
    }
  }
}
