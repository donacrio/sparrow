mod fixed_size;

pub use self::fixed_size::FixedSizeStorage;

use std::hash::Hash;

pub enum Storage<Key, Value> {
  FixedSizeStorage(FixedSizeStorage<Key, Value>),
}

pub enum StorageType {
  FixedSizeStorage,
}

pub fn get_storage<Key, Value>(storage_type: StorageType, capacity: usize) -> Storage<Key, Value>
where
  Key: Eq + Hash,
{
  match storage_type {
    StorageType::FixedSizeStorage => {
      Storage::FixedSizeStorage(FixedSizeStorage::<Key, Value>::new(capacity))
    }
  }
}

pub trait Cache<Key, Value> {
  fn get(&self, key: Key) -> Option<&Value>;
  fn put(&mut self, key: Key, v: Value) -> Result<(), &str>;
  fn delete(&mut self, key: Key) -> Option<Value>;
}

impl<Key, Value> Cache<Key, Value> for Storage<Key, Value>
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
