use crate::http::parser::parse_request;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use super::thread_pool::ThreadPool;

pub struct WebServer {
    addr: String,
    port: u32,
    pool: ThreadPool,
    listener: TcpListener,
}

impl WebServer {
    pub fn new(addr: String, port: u32) -> WebServer {
        assert!(port > 0);

        let mut port = port;
        let listener: TcpListener;

        loop {
            let l = TcpListener::bind(format!("{}:{}", addr, port));

            match l {
                Ok(i) => {
                    listener = i;
                    break;
                }
                Err(e) => {
                    println!("Could not bind to port. Error {}. Trying next available", e);
                    port += 1;
                }
            };
        }

        let p = ThreadPool::new(16);

        println!("Creating server at address: {}:{}", addr, port);

        WebServer {
            addr: addr,
            port: port,
            pool: p,
            listener: listener,
        }
    }

    pub fn start_server(&self) // Add result type?
    {
        for incoming in self.listener.incoming() {
            match incoming {
                Ok(connection) => {
                    println!("Got connection");
                    self.pool
                        .execute(move || WebServer::handle_connection(&connection));
                    println!("Thread gone")
                }
                Err(e) => println!("Failed to establish connection. Error {}", e),
            }
        }
    }

    fn handle_connection(mut stream: &TcpStream) {
        let mut buf_reader = BufReader::new(stream);

        println!("Here 0");
        let http_request: Vec<_> = (&mut buf_reader)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut request_body: Vec<u8> = Vec::new();

        let content_length = http_request
            .iter()
            .find(|line| line.to_lowercase().starts_with("content-length"))
            .and_then(|line| line.split(": ").nth(1))
            .and_then(|r| r.parse::<usize>().ok());

        if let Some(length) = content_length {
            let mut body = vec![0; length];
            if let Ok(_) = buf_reader.read_exact(&mut body) {
                request_body = body;
            }
        }

        println!("Request {:#?}", http_request);
        println!("Request body: {:?}", String::from_utf8_lossy(&request_body));

        let _parse_result = parse_request(&http_request, &request_body);

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        match stream.write_all(response.as_bytes()) {
            Ok(_) => println!("Responded"),
            Err(_) => print!("Failed to respond"),
        }
    }
}
