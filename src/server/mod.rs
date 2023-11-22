pub mod request;
pub mod response;
pub mod utils;
use request::HTTPRequest;
use response::HTTPResponse;
use std::io::prelude::*;
use std::net::TcpListener;

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        return Self { port };
    }

    pub fn serve(&self) {
        let listener = TcpListener::
            bind("127.0.0.1:".to_owned() + &self.port.to_string()).unwrap();

        println!("Listening on port: {}", self.port);

        for stream in listener.incoming() {
            let mut _stream = stream.unwrap();

            let mut buf = vec![0; 32768];
            _stream.read(&mut buf).unwrap();

            println!("Req: {}", String::from_utf8(buf.to_vec()).unwrap());
            let req = HTTPRequest::parse(String::from_utf8(buf.to_vec()).
                                         unwrap().as_str());

            let res = HTTPResponse::new(
                "HTTP/1.1".to_owned(),
                "200 OK".to_owned(),
                "text/plain".to_owned(),
                String::new(),
            );

            println!("{}", res.construct());
        }
    }
}
