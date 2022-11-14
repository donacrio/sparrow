use std::{
  io::{self, BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

pub fn run_tcp_server(addr: &str) -> Result<(), io::Error> {
  let listener = TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        if let Err(err) = handle_connection(stream) {
          eprintln!("{}", err);
        }
      }
      Err(err) => eprintln!("{}", err),
    }
  }
  Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), io::Error> {
  let buf_reader = BufReader::new(&mut stream);
  let request = buf_reader
    .lines()
    .map(|result| result.unwrap_or_else(|_| "".to_string()))
    .take_while(|line| !line.is_empty())
    .collect::<Vec<_>>();

  println!("Request: {:#?}", request);

  let response = "HTTP/1.1 200 OK\r\n\r\n";

  stream.write_all(response.as_bytes())?;
  Ok(())
}
