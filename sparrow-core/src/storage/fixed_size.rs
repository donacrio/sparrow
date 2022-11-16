use crate::traits;

use super::Storage;
use std::collections::HashMap;

pub struct FixedSizeStorage<K, V> {
  capacity: usize,
  map: HashMap<K, V>,
}

impl<K, V> FixedSizeStorage<K, V>
where
  K: traits::StorageKey,
{
  pub fn new(capacity: usize) -> Self {
    Self {
      capacity,
      map: HashMap::<K, V>::new(),
    }
  }
}

impl<K, V> Storage<K, V> for FixedSizeStorage<K, V>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  fn get(&self, key: K) -> Option<&V> {
    self.map.get(&key)
  }

  fn put(&mut self, key: K, value: V) -> Result<(), &str> {
    if self.map.keys().count() < self.capacity || self.map.contains_key(&key) {
      self.map.insert(key, value);
      return Ok(());
    }
    Err("Storage is full")
  }

  fn delete(&mut self, key: K) -> Option<V> {
    self.map.remove(&key)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::{fixture, rstest};

  type K = i32;
  type V = i32;

  const CAPACITY: usize = 2;
  const TEST_KEY_0: i32 = 0;
  const TEST_VALUE_0: i32 = 0;
  const TEST_KEY_1: i32 = 1;
  const TEST_VALUE_1: i32 = 1;

  #[fixture]
  fn storage() -> FixedSizeStorage<K, V> {
    let mut storage = FixedSizeStorage::new(CAPACITY);
    storage.put(TEST_KEY_0, TEST_VALUE_0).unwrap();
    storage
  }

  #[fixture]
  fn storage_full() -> FixedSizeStorage<K, V> {
    let mut storage = FixedSizeStorage::new(CAPACITY);
    storage.put(TEST_KEY_0, TEST_VALUE_0).unwrap();
    storage.put(TEST_KEY_1, TEST_VALUE_1).unwrap();
    storage
  }

  #[test]
  fn test_new() {
    let storage = FixedSizeStorage::<K, V>::new(1);
    assert_eq!(storage.capacity, 1);
    assert_eq!(storage.map.len(), 0);
  }

  #[rstest]
  fn test_get_key_exists(storage: FixedSizeStorage<K, V>) {
    assert_eq!(storage.get(TEST_KEY_0), Some(&TEST_VALUE_0));
  }

  #[rstest]
  fn test_get_key_absent(storage: FixedSizeStorage<K, V>) {
    assert_eq!(storage.get(TEST_KEY_1), None);
  }

  #[rstest]
  fn test_put(mut storage: FixedSizeStorage<K, V>) {
    assert_eq!(storage.get(1), None);
    let result = storage.put(1, 1);
    assert_eq!(result, Ok(()));
    assert_eq!(storage.get(1), Some(&1));
  }

  #[rstest]
  fn test_put_full_key_exists(mut storage_full: FixedSizeStorage<K, V>) {
    assert_eq!(storage_full.get(TEST_KEY_0), Some(&TEST_VALUE_0));
    let result = storage_full.put(TEST_KEY_0, TEST_VALUE_1);
    assert_eq!(result, Ok(()));
    assert_eq!(storage_full.get(TEST_KEY_0), Some(&TEST_VALUE_1));
  }

  #[rstest]
  fn test_put_full_key_absent(mut storage_full: FixedSizeStorage<K, V>) {
    let result = storage_full.put(5, 5);
    assert_eq!(result, Err("Storage is full"));
  }

  #[rstest]
  fn test_delete(mut storage: FixedSizeStorage<K, V>) {
    assert_eq!(storage.get(TEST_KEY_0), Some(&TEST_VALUE_0));
    storage.delete(TEST_KEY_0);
    assert_eq!(storage.get(TEST_KEY_0), None);
  }
}
