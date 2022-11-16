mod error;

pub use self::error::CommandError;
use std::str::FromStr;

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
          let key = split
            .next()
            .map(|key| key.parse())
            .ok_or(CommandError::Malformed)?
            .map_err(|_| CommandError::Malformed)?;
          Ok(Command::Get(key))
        }
        "PUT" => {
          let key = split
            .next()
            .map(|key| key.parse())
            .ok_or(CommandError::Malformed)?
            .map_err(|_| CommandError::Malformed)?;
          let value = split
            .next()
            .map(|value| value.parse())
            .ok_or(CommandError::Malformed)?
            .map_err(|_| CommandError::Malformed)?;
          Ok(Command::Put(key, value))
        }
        "DEL" => {
          let key = split
            .next()
            .map(|key| key.parse())
            .ok_or(CommandError::Malformed)?
            .map_err(|_| CommandError::Malformed)?;
          Ok(Command::Delete(key))
        }
        _ => Err(CommandError::NotFound),
      },
      None => Err(CommandError::Malformed),
    }
  }
}
