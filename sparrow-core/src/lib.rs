mod command;
mod engine;
mod error;
mod storage;
mod tcp;
mod traits;

pub use engine::Engine;
pub use error::Result;
pub use storage::StorageType;

// TODO: this is a workaround until proper trait implementation is done
impl traits::StorageKey for i32 {}
impl traits::StorageValue for i32 {}
