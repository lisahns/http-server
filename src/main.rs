fn main() {
// let string = String::from("127.0.0.1:8080");
// // everything after the 10th byte, eg emojis are more than one byte each, not good practice
// // better is find, which gives back space in a string
// let string_slice = &string[10..];

// dbg!(&string);
// dbg!(string_slice);

    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    // self is like this
    fn run(self) {
        println!("Listenin on {}", self.addr)
    }
}

 struct Request {
        path: String,
        query_string: Option<String>,
        method: Method,
    }

    enum Method {
        GET,
        POST,
        PUT,
        DELETE,
        HEAD,
        CONNECT,
        OPTIONS,
        TRACE,
        PATCH
    }