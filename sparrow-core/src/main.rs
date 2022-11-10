mod engine;
mod storage;

use engine::Engine;
use storage::FixedSizeStorage;

fn main() {
  Engine::<i32, i32, FixedSizeStorage<i32, i32>>::new(10);
}
