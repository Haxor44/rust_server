use std::{io::Read, io::Write, net::TcpListener};
use crate::http::request;
use crate::http::request::ParseError;
use crate::http::request::Request;
use crate::http::response;
use super::http::{Response,StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_Request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request {}",e);
        Response::new(StatusCode::BadRequest, None)
    }
}
    //holds definition of struct
    pub struct Server {
        addr: String,
    }

    //holds implementation of struct
    impl Server {
        pub fn new(addr:String)-> Server{
            Server {
                addr:addr
            }
        }

        pub fn run(self, mut handler: impl Handler ){
            println!("Listening on {}", self.addr);
            let listener = TcpListener::bind(&self.addr).unwrap();

            loop {
                match listener.accept() {
                    Ok((mut stream, addr)) => {
                        println!("OK");
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_)=>{
                                println!("Received a request:{}",String::from_utf8_lossy(&buffer));
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) =>  handler.handle_bad_Request(&e)
                                };
                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response: {}",e);
                                }
                            },
                            Err(e)=> println!("Failed to read from connection: {}",e),
                        }
                    },
                    Err(e) => println!("Failed to establish connection: {}",e),
                }
            }
        }
    }
