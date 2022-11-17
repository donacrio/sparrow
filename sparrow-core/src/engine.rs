use std::str::FromStr;

use crate::{
  command::Command,
  error::Result,
  storage::{create_storage, Storage, StorageEnum, StorageType},
  tcp, traits,
};

pub struct Engine<K, V> {
  storage: StorageEnum<K, V>,
}

impl<K, V> Engine<K, V>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  pub fn new(storage_type: StorageType, capacity: usize) -> Self {
    Self {
      storage: create_storage(storage_type, capacity),
    }
  }

  pub fn run(&mut self, addr: &str) -> Result<()> {
    let mut handle_request = |request: String| self.execute(Command::from_str(&request)?);
    tcp::run_tcp_server::<K, V, _>(addr, &mut handle_request)
  }

  fn execute(&mut self, command: Command<K, V>) -> Result<String> {
    match command {
      Command::Get(key) => Ok(
        self
          .storage
          .get(key)
          .map_or("None".to_owned(), |value| value.to_string()),
      ),
      Command::Put(key, value) => self
        .storage
        .put(key, value)
        .map(|value| value.map_or("None".to_owned(), |value| value.to_string()))
        .map_err(|err| err.into()),
      Command::Delete(key) => Ok(
        self
          .storage
          .delete(key)
          .map_or("None".to_owned(), |value| value.to_string()),
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::fixture;

  type K = i32;
  type V = i32;

  const CAPACITY: usize = 2;
  const TEST_KEY_0: i32 = 0;
  const TEST_VALUE_0: i32 = 0;
  const TEST_KEY_1: i32 = 1;
  const TEST_VALUE_1: i32 = 1;

  #[fixture]
  fn engine() -> Engine<K, V> {
    let mut engine = Engine::new(StorageType::FixedSize, CAPACITY);
    engine
      .execute(Command::Put(TEST_KEY_0, TEST_VALUE_0))
      .unwrap();
    engine
  }

  mod get {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn key_exists(mut engine: Engine<K, V>) {
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_0)).unwrap(),
        TEST_VALUE_0.to_string()
      );
    }

    #[rstest]
    fn key_absent(mut engine: Engine<K, V>) {
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_1)).unwrap(),
        "None".to_string()
      );
    }
  }

  mod put {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn key_exists(mut engine: Engine<K, V>) {
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_0)).unwrap(),
        TEST_VALUE_0.to_string()
      );
      assert_eq!(
        engine
          .execute(Command::Put(TEST_KEY_0, TEST_VALUE_1))
          .unwrap(),
        TEST_VALUE_0.to_string()
      );
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_0)).unwrap(),
        TEST_VALUE_1.to_string()
      );
    }

    #[rstest]
    fn key_absent(mut engine: Engine<K, V>) {
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_1)).unwrap(),
        "None".to_owned()
      );
      assert_eq!(
        engine
          .execute(Command::Put(TEST_KEY_1, TEST_VALUE_1))
          .unwrap(),
        "None".to_owned()
      );
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_1)).unwrap(),
        TEST_VALUE_1.to_string()
      );
    }
  }

  mod delete {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn key_exists(mut engine: Engine<K, V>) {
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_0)).unwrap(),
        TEST_VALUE_0.to_string()
      );
      assert_eq!(
        engine.execute(Command::Delete(TEST_KEY_0)).unwrap(),
        TEST_VALUE_0.to_string()
      );
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_0)).unwrap(),
        "None".to_owned()
      );
    }

    #[rstest]
    fn key_absent(mut engine: Engine<K, V>) {
      assert_eq!(
        engine.execute(Command::Get(TEST_KEY_1)).unwrap(),
        "None".to_owned()
      );
      assert_eq!(
        engine.execute(Command::Delete(TEST_KEY_1)).unwrap(),
        "None".to_owned()
      );
    }
  }
}
