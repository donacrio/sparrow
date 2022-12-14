mod error;

pub use self::error::CommandError;
use std::str::{FromStr, Split};
pub enum Command<Key, Value> {
  Get(Key),
  Put(Key, Value),
  Delete(Key),
}

impl<Key, Value> FromStr for Command<Key, Value>
where
  Key: FromStr,
  Value: FromStr,
{
  type Err = CommandError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split(' ');
    match split.next() {
      Some(command) => match command {
        "GET" => {
          let key = parse_parameter(&mut split)?;
          Ok(Command::Get(key))
        }
        "PUT" => {
          let key = parse_parameter(&mut split)?;
          let value = parse_parameter(&mut split)?;
          Ok(Command::Put(key, value))
        }
        "DEL" => {
          let key = parse_parameter(&mut split)?;
          Ok(Command::Delete(key))
        }
        unknown => Err(CommandError::NotFound(unknown.to_owned())),
      },
      None => Err(CommandError::Malformed("No command provided".to_owned())),
    }
  }
}

fn parse_parameter<T>(split: &mut Split<char>) -> Result<T, CommandError>
where
  T: FromStr,
{
  split
    .next()
    .map(|param| param.parse())
    .ok_or_else(|| CommandError::Malformed("Missing parameter".to_owned()))?
    .map_err(|_| CommandError::Malformed("Cannot parse parameter".to_owned()))
}

// TODO: add tests
