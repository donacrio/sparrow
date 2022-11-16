mod command;
mod storage;
mod tcp_server;

use storage::{create_storage, StorageEnum, StorageType};
use tcp_server::run_tcp_server;

pub fn run(capacity: usize, addr: &str) -> Result<(), std::io::Error> {
  let storage: StorageEnum<i32, i32> = create_storage(StorageType::FixedSizeStorage, capacity);
  run_tcp_server(addr, storage)
}
