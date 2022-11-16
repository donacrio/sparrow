mod fixed_size;

pub use self::fixed_size::FixedSizeStorage;

use std::hash::Hash;

pub enum StorageEnum<Key, Value> {
  FixedSizeStorage(FixedSizeStorage<Key, Value>),
}

pub enum StorageType {
  FixedSizeStorage,
}

pub fn create_storage<Key, Value>(
  storage_type: StorageType,
  capacity: usize,
) -> StorageEnum<Key, Value>
where
  Key: Eq + Hash,
{
  match storage_type {
    StorageType::FixedSizeStorage => {
      StorageEnum::FixedSizeStorage(FixedSizeStorage::<Key, Value>::new(capacity))
    }
  }
}

pub trait Storage<Key, Value> {
  fn get(&self, key: Key) -> Option<&Value>;
  fn put(&mut self, key: Key, value: Value) -> Result<(), &str>;
  fn delete(&mut self, key: Key) -> Option<Value>;
}

impl<Key, Value> Storage<Key, Value> for StorageEnum<Key, Value>
where
  Key: Eq + Hash,
{
  fn get(&self, key: Key) -> Option<&Value> {
    match self {
      Self::FixedSizeStorage(inner) => inner.get(key),
    }
  }

  fn put(&mut self, key: Key, value: Value) -> Result<(), &str> {
    match self {
      Self::FixedSizeStorage(inner) => inner.put(key, value),
    }
  }

  fn delete(&mut self, key: Key) -> Option<Value> {
    match self {
      Self::FixedSizeStorage(inner) => inner.delete(key),
    }
  }
}
