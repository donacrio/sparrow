use crate::storage::{get_storage, Storage, StorageType};
use std::hash::Hash;

pub struct Engine<K, V>
where
  K: Eq + Hash,
{
  _storage: Storage<K, V>,
}

impl<K, V> Engine<K, V>
where
  K: Eq + Hash,
{
  pub fn new(capacity: usize) -> Self {
    Self {
      _storage: get_storage(StorageType::FixedSizeStorage, capacity),
    }
  }
}
