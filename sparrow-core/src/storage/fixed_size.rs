use super::Storage;
use std::{collections::HashMap, hash::Hash};

pub struct FixedSizeStorage<K, V> {
  capacity: usize,
  map: HashMap<K, V>,
}

impl<K, V> Storage<K, V> for FixedSizeStorage<K, V>
where
  K: Eq + Hash,
{
  fn new(capacity: usize) -> Self {
    Self {
      capacity,
      map: HashMap::<K, V>::new(),
    }
  }

  fn get(&self, key: K) -> Option<&V> {
    self.map.get(&key)
  }

  fn put(&mut self, k: K, v: V) -> Result<(), &str> {
    match self.map.keys().count() < self.capacity {
      true => {
        self.map.insert(k, v);
        self.capacity += 1;
        Ok(())
      }
      false => Err("Storage is full"),
    }
  }

  fn delete(&mut self, k: K) -> Option<V> {
    self.map.remove(&k)
  }
}
