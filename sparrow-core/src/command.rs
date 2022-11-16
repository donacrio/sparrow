use std::str::FromStr;

pub enum Command<Key, Value> {
  Get(Key),
  Put(Key, Value),
  Delete(Key),
}

pub enum Error<Key, Value>
where
  Key: FromStr,
  Value: FromStr,
{
  NotFound,
  Malformed,
  ParseKey(<Key as FromStr>::Err),
  ParseValue(<Value as FromStr>::Err),
}

impl<Key, Value> FromStr for Command<Key, Value>
where
  Key: FromStr,
  Value: FromStr,
{
  type Err = Error<Key, Value>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split(' ');
    match split.next() {
      Some(command) => match command {
        "GET" => {
          let key = split
            .next()
            .map(|key| key.parse())
            .ok_or(Error::Malformed)?
            .map_err(|err| Error::ParseKey(err))?;
          Ok(Command::Get(key))
        }
        "PUT" => {
          let key = split
            .next()
            .map(|key| key.parse())
            .ok_or(Error::Malformed)?
            .map_err(|err| Error::ParseKey(err))?;
          let value = split
            .next()
            .map(|value| value.parse())
            .ok_or(Error::Malformed)?
            .map_err(|err| Error::ParseValue(err))?;
          Ok(Command::Put(key, value))
        }
        "DEL" => {
          let key = split
            .next()
            .map(|key| key.parse())
            .ok_or(Error::Malformed)?
            .map_err(|err| Error::ParseKey(err))?;
          Ok(Command::Delete(key))
        }
        _ => Err(Error::NotFound),
      },
      None => Err(Error::Malformed),
    }
  }
}
