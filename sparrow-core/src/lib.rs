mod engine;
mod storage;
mod tcp_server;

use engine::Engine;
use tcp_server::run_tcp_server;

pub fn run(capacity: usize, addr: &str) -> Result<(), std::io::Error> {
  let _engine = Engine::<i32, i32>::new(capacity);
  run_tcp_server(addr)
}
