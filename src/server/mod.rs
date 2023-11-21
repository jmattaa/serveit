mod http;
use http::HTTPRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Server {
        return Server {
            port
        };
    }

    pub fn serve(&self) {
        let listener = 
            TcpListener::bind("127.0.0.1:".to_owned() + &self.port.to_string())
            .unwrap();

        println!("Listening on port: {}", self.port);

        for stream in listener.incoming() {
            let mut _stream = stream.unwrap();

            let mut buf = vec![0; 32768];
            _stream.read(&mut buf).unwrap();

            println!("Res: {}", String::from_utf8(buf.to_vec()).unwrap());
        }
    }
}
