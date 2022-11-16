mod fixed_size;

use crate::traits;

pub use self::fixed_size::FixedSizeStorage;

pub enum StorageEnum<K, V> {
  FixedSizeStorage(FixedSizeStorage<K, V>),
}

pub enum StorageType {
  FixedSizeStorage,
}

pub fn create_storage<K, V>(storage_type: StorageType, capacity: usize) -> StorageEnum<K, V>
where
  K: traits::StorageKey,
{
  match storage_type {
    StorageType::FixedSizeStorage => {
      StorageEnum::FixedSizeStorage(FixedSizeStorage::<K, V>::new(capacity))
    }
  }
}

pub trait Storage<K, V>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  fn get(&self, key: K) -> Option<&V>;
  fn put(&mut self, key: K, value: V) -> Result<(), &str>;
  fn delete(&mut self, key: K) -> Option<V>;
}

impl<K, V> Storage<K, V> for StorageEnum<K, V>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  fn get(&self, key: K) -> Option<&V> {
    match self {
      Self::FixedSizeStorage(inner) => inner.get(key),
    }
  }

  fn put(&mut self, key: K, value: V) -> Result<(), &str> {
    match self {
      Self::FixedSizeStorage(inner) => inner.put(key, value),
    }
  }

  fn delete(&mut self, key: K) -> Option<V> {
    match self {
      Self::FixedSizeStorage(inner) => inner.delete(key),
    }
  }
}
