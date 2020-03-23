
use std::net::TcpListener;

pub fn start() {
  let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

  for stream in listener.incoming() {
    let message = stream.unwrap();
    println!("Connection established");
  }
}