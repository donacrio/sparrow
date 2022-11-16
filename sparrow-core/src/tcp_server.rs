use crate::{
  command::Command,
  error::Result,
  storage::{Storage, StorageEnum},
  traits,
};
use std::{
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
  str::FromStr,
};

pub fn run_tcp_server<K, V>(addr: &str, mut storage: StorageEnum<K, V>) -> Result<()>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  let listener = TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        if handle_connection(stream, &mut storage).is_err() {
          //TODO: Implement Display for errors
          eprintln!("an error occured");
        }
      }
      Err(err) => eprintln!("{}", err),
    }
  }
  Ok(())
}

fn handle_connection<K, V>(mut stream: TcpStream, storage: &mut StorageEnum<K, V>) -> Result<()>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
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
    .map(|request| match handle_request(request, storage) {
      Ok(response) => response,
      Err(_) => "Error".to_owned(),
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

fn handle_request<K, V>(request: String, storage: &mut StorageEnum<K, V>) -> Result<String>
where
  K: traits::StorageKey,
  V: traits::StorageValue,
{
  match Command::from_str(&request)? {
    Command::Get(key) => Ok(
      storage
        .get(key)
        .map_or("None".to_owned(), |value| value.to_string()),
    ),
    Command::Put(key, value) => storage
      .put(key, value)
      .map(|value| value.map_or("None".to_owned(), |value| value.to_string()))
      .map_err(|err| err.into()),
    Command::Delete(key) => Ok(
      storage
        .delete(key)
        .map(|_| "OK".to_owned())
        .map_or("None".to_owned(), |value| value),
    ),
  }
}
