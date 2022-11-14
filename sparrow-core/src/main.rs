mod engine;
mod storage;

use engine::Engine;

fn main() {
  Engine::<i32, i32>::new(10);
}
