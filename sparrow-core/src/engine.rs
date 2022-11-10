use std::{hash::Hash, marker::PhantomData};

use crate::storage::Storage;

pub struct Engine<K, V, S>
where
  K: Eq + Hash,
  S: Storage<K, V>,
{
  _storage: S,
  _k: PhantomData<K>,
  _v: PhantomData<V>,
}

impl<K, V, S> Engine<K, V, S>
where
  K: Eq + Hash,
  S: Storage<K, V>,
{
  pub fn new(capacity: usize) -> Self {
    Self {
      _storage: S::new(capacity),
      _k: PhantomData {},
      _v: PhantomData {},
    }
  }
}
