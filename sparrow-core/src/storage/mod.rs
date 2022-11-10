mod fixed_size;

pub use self::fixed_size::FixedSizeStorage;

pub trait Storage<K, V> {
  fn new(capacity: usize) -> Self;
  fn get(&self, k: K) -> Option<&V>;
  fn put(&mut self, k: K, v: V) -> Result<(), &str>;
  fn delete(&mut self, k: K) -> Option<V>;
}
