mod command;
mod error;
mod storage;
mod tcp_server;
mod traits;

use crate::error::Result;
use storage::{create_storage, StorageEnum, StorageType};
use tcp_server::run_tcp_server;

// TODO: this is a workaround until proper trait implementation is done
impl traits::StorageKey for i32 {}
impl traits::StorageValue for i32 {}

pub fn run(capacity: usize, addr: &str) -> Result<()> {
  let storage: StorageEnum<i32, i32> = create_storage(StorageType::FixedSize, capacity);
  run_tcp_server(addr, storage)
}
