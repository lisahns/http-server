use crate::http::Request;
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::Read;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }
    // self is like this
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        // unwrap will return ok result, and will terminate if it returns error

        // loop is infinite loop - can label loop eg 'outer: loop
        loop {
            match listener.accept() {
                // like cases in switch statement
                Ok((mut stream, _)) => {
                    // creates a value for 1024 elements this will be 1024 bytes
                    // buffer to handle requests? memory?
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // replaces invalid utf8
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // reference to buffer, converted to array
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => print!("Failed to parse request, error: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
                // _ can be used as catch all for match statement
            }
        }
    }
}

// file server.rs is the same as mod server {}
// by default modules are always private

// two types of errors: recoverable, and not recoverable
