// import module
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// handle client steam implement echo feature
fn handle_stream(mut stream: TcpStream) {
  // loop read
  loop {
    // read buffer
    let mut buf = [0; 1024];
    // read and use match check error
    match stream.read(&mut buf) {
      Ok(n) => {
        // if empty then break
        if n == 0 {
          break;
        }
        // write response use read content
        stream.write(&["Server response: ".as_bytes(), &buf[0..n]].concat()).unwrap();
      }
      // process error
      Err(err) => {
        // panic error
        panic!(err);
      }
    }
  }
}

// main
fn main() -> std::io::Result<()> {
  // bind 8888 port to listen client connect
  let listener = TcpListener::bind("localhost:8888")?;

  // blocking listen client connection
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        // use thread process connect
        thread::spawn(move || {
          // handle client stream
          handle_stream(stream);
        });
      }
      // check error
      Err(e) => {
        println!("error {}", e);
      }
    }
  }

  Ok(())
}

