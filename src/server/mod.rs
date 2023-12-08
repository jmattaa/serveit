pub mod request;
pub mod response;
pub mod utils;
use request::HTTPRequest;
use response::HTTPResponse;
use std::fs;
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

            let path: String = req.parse_path();
            let contents: String = match fs::read_to_string(path) {
                Ok(contents) => {
                    contents
                },
                Err(_) => {
                    // give 'em an invalid response
                    // ain't the best way i guess but works for now
                    _stream.write(HTTPResponse::new(
                            req.version().to_owned(),
                            "404 NOT FOUND".to_owned(),
                            "text/html".to_owned(),
                            utils::HTML404PAGE.to_owned()
                            ).construct().as_bytes()
                        ).unwrap();

                    String::new()
                },
            };

            let res = HTTPResponse::new(
                req.version().to_owned(),
                "200 OK".to_owned(),
                "text/html".to_owned(),
                contents
            );

            _stream.write(res.construct().as_bytes()).unwrap();
            _stream.flush().unwrap();
        }
    }
}
