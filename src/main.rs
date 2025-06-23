use http::Request;
use http::Method;
use server::Server;

mod server;
mod http;

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
