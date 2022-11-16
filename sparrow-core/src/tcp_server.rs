use std::{
  hash::Hash,
  io::{self, BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
  str::FromStr,
};

use crate::{
  command::{self, Command},
  storage::{Storage, StorageEnum},
};

pub fn run_tcp_server<Key, Value>(
  addr: &str,
  mut storage: StorageEnum<Key, Value>,
) -> Result<(), io::Error>
where
  Key: FromStr + ToString + Eq + Hash,
  Value: FromStr + ToString,
{
  let listener = TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        if let Err(err) = handle_connection(stream, &mut storage) {
          eprintln!("{}", err);
        }
      }
      Err(err) => eprintln!("{}", err),
    }
  }
  Ok(())
}

fn handle_connection<Key, Value>(
  mut stream: TcpStream,
  storage: &mut StorageEnum<Key, Value>,
) -> Result<(), io::Error>
where
  Key: FromStr + ToString + Eq + Hash,
  Value: FromStr + ToString,
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

fn handle_request<Key, Value>(
  request: String,
  storage: &mut StorageEnum<Key, Value>,
) -> Result<String, command::Error<Key, Value>>
where
  Key: FromStr + ToString + Eq + Hash,
  Value: FromStr + ToString,
{
  match Command::from_str(&request)? {
    Command::Get(key) => Ok(
      storage
        .get(key)
        .map_or("None".to_owned(), |value| value.to_string()),
    ),
    Command::Put(key, value) => storage
      .put(key, value)
      .map(|_| "OK".to_owned())
      .map_err(|_| command::Error::NotFound),
    Command::Delete(key) => Ok(
      storage
        .delete(key)
        .map(|_| "OK".to_owned())
        .map_or("None".to_owned(), |value| value),
    ),
  }
}
