// TODO: Add more enum member for better error description
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum CommandError {
  NotFound,
  Malformed,
}
