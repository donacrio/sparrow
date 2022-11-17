use crate::{error::Result, traits};
use std::{
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

pub fn run_tcp_server<K, V, F>(addr: &str, handle_request: &mut F) -> Result<()>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
  F: FnMut(String) -> Result<String>,
{
  let listener = TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        if let Err(err) = handle_connection::<K, V, _>(stream, handle_request) {
          eprintln!("{}", err);
        }
      }
      Err(err) => eprintln!("{}", err),
    }
  }
  Ok(())
}

fn handle_connection<K, V, F>(mut stream: TcpStream, handle_request: &mut F) -> Result<()>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
  F: FnMut(String) -> Result<String>,
{
  let buf_reader = BufReader::new(&mut stream);
  let request: Vec<String> = buf_reader
    .lines()
    .map(|result| result.unwrap_or_else(|_| "".to_owned()))
    .take_while(|line| !line.is_empty())
    .collect();

  println!("Request: {:#?}", request);

  let responses: Vec<String> = request
    .into_iter()
    .map(|request| match handle_request(request) {
      Ok(response) => response,
      Err(err) => err.to_string(),
    })
    .collect();

  println!("Response: {:#?}", responses);

  responses.iter().for_each(|response| {
    if let Err(err) = stream.write_all(response.as_bytes()) {
      eprintln!("{}", err);
    }
  });

  Ok(())
}
