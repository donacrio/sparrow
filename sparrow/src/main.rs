use sparrow_core::{Engine, Result, StorageType};

const CAPACITY: usize = 1000;
const ADDR: &str = "127.0.0.1:3000";

fn main() {
  match run(CAPACITY, ADDR) {
    Ok(_) => std::process::exit(0),
    Err(err) => {
      eprintln!("{}", err);
      std::process::exit(1)
    }
  };
}

pub fn run(capacity: usize, addr: &str) -> Result<()> {
  Engine::<i32, i32>::new(StorageType::FixedSize, capacity).run(addr)
}
