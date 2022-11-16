mod error;
mod fixed_size;

pub use self::error::StorageError;
pub use self::fixed_size::FixedSizeStorage;
use crate::traits;

pub enum StorageEnum<K, V> {
  FixedSize(FixedSizeStorage<K, V>),
}

pub enum StorageType {
  FixedSize,
}

pub fn create_storage<K, V>(storage_type: StorageType, capacity: usize) -> StorageEnum<K, V>
where
  K: traits::StorageKey,
{
  match storage_type {
    StorageType::FixedSize => StorageEnum::FixedSize(FixedSizeStorage::<K, V>::new(capacity)),
  }
}

pub trait Storage<K, V>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  fn get(&self, key: K) -> Option<&V>;
  fn put(&mut self, key: K, value: V) -> Result<Option<V>, StorageError>;
  fn delete(&mut self, key: K) -> Option<V>;
}

impl<K, V> Storage<K, V> for StorageEnum<K, V>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  fn get(&self, key: K) -> Option<&V> {
    match self {
      Self::FixedSize(inner) => inner.get(key),
    }
  }

  fn put(&mut self, key: K, value: V) -> Result<Option<V>, StorageError> {
    match self {
      Self::FixedSize(inner) => inner.put(key, value),
    }
  }

  fn delete(&mut self, key: K) -> Option<V> {
    match self {
      Self::FixedSize(inner) => inner.delete(key),
    }
  }
}
