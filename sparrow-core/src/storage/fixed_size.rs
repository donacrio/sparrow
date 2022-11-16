use super::Storage;
use std::{collections::HashMap, hash::Hash};

pub struct FixedSizeStorage<Key, Value> {
  capacity: usize,
  map: HashMap<Key, Value>,
}

impl<Key, Value> FixedSizeStorage<Key, Value>
where
  Key: Eq + Hash,
{
  pub fn new(capacity: usize) -> Self {
    Self {
      capacity,
      map: HashMap::<Key, Value>::new(),
    }
  }
}

impl<Key, Value> Storage<Key, Value> for FixedSizeStorage<Key, Value>
where
  Key: Eq + Hash,
{
  fn get(&self, key: Key) -> Option<&Value> {
    self.map.get(&key)
  }

  fn put(&mut self, key: Key, value: Value) -> Result<(), &str> {
    if self.map.keys().count() < self.capacity || self.map.contains_key(&key) {
      self.map.insert(key, value);
      return Ok(());
    }
    Err("Storage is full")
  }

  fn delete(&mut self, key: Key) -> Option<Value> {
    self.map.remove(&key)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::{fixture, rstest};

  type Key = i32;
  type Value = i32;

  const CAPACITY: usize = 2;
  const TEST_KEY_0: i32 = 0;
  const TEST_VALUE_0: i32 = 0;
  const TEST_KEY_1: i32 = 1;
  const TEST_VALUE_1: i32 = 1;

  #[fixture]
  fn storage() -> FixedSizeStorage<Key, Value> {
    let mut storage = FixedSizeStorage::new(CAPACITY);
    storage.put(TEST_KEY_0, TEST_VALUE_0).unwrap();
    storage
  }

  #[fixture]
  fn storage_full() -> FixedSizeStorage<Key, Value> {
    let mut storage = FixedSizeStorage::new(CAPACITY);
    storage.put(TEST_KEY_0, TEST_VALUE_0).unwrap();
    storage.put(TEST_KEY_1, TEST_VALUE_1).unwrap();
    storage
  }

  #[test]
  fn test_new() {
    let storage = FixedSizeStorage::<Key, Value>::new(1);
    assert_eq!(storage.capacity, 1);
    assert_eq!(storage.map.len(), 0);
  }

  #[rstest]
  fn test_get_key_exists(storage: FixedSizeStorage<Key, Value>) {
    assert_eq!(storage.get(TEST_KEY_0), Some(&TEST_VALUE_0));
  }

  #[rstest]
  fn test_get_key_absent(storage: FixedSizeStorage<Key, Value>) {
    assert_eq!(storage.get(TEST_KEY_1), None);
  }

  #[rstest]
  fn test_put(mut storage: FixedSizeStorage<Key, Value>) {
    assert_eq!(storage.get(1), None);
    let result = storage.put(1, 1);
    assert_eq!(result, Ok(()));
    assert_eq!(storage.get(1), Some(&1));
  }

  #[rstest]
  fn test_put_full_key_exists(mut storage_full: FixedSizeStorage<Key, Value>) {
    assert_eq!(storage_full.get(TEST_KEY_0), Some(&TEST_VALUE_0));
    let result = storage_full.put(TEST_KEY_0, TEST_VALUE_1);
    assert_eq!(result, Ok(()));
    assert_eq!(storage_full.get(TEST_KEY_0), Some(&TEST_VALUE_1));
  }

  #[rstest]
  fn test_put_full_key_absent(mut storage_full: FixedSizeStorage<Key, Value>) {
    let result = storage_full.put(5, 5);
    assert_eq!(result, Err("Storage is full"));
  }

  #[rstest]
  fn test_delete(mut storage: FixedSizeStorage<Key, Value>) {
    assert_eq!(storage.get(TEST_KEY_0), Some(&TEST_VALUE_0));
    storage.delete(TEST_KEY_0);
    assert_eq!(storage.get(TEST_KEY_0), None);
  }
}
