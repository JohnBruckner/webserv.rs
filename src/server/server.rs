use super::thread_pool::ThreadPool;
use crate::http::parser::{parse_request};
use std::collections::HashMap;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};
use crate::request::request::RequestMethod;

type EndpointMethod = Box<dyn FnOnce() + Send + 'static>;

pub struct WebServer {
    addr: String,
    port: u32,
    pool: ThreadPool,
    listener: TcpListener,
    route_map: HashMap<String, Endpoint>,
}

struct Endpoint {
    request_method: RequestMethod,
    execution_method: EndpointMethod,
}

impl Endpoint {
    fn new(request_method: RequestMethod, execution_method: EndpointMethod) -> Endpoint {
        Endpoint {
            request_method,
            execution_method,
        }
    }
}

pub struct WebServerBuilder {
    addr: String,
    port: u32,
    port_auto_discover: bool,
    thread_count: usize,
    route_map: HashMap<String, Endpoint>,
}

impl WebServerBuilder {
    pub fn new() -> WebServerBuilder {
        WebServerBuilder {
            addr: "127.0.0.1".to_string(),
            port: 8080,
            port_auto_discover: false,
            thread_count: 16,
            route_map: HashMap::new(),
        }
    }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.addr = addr.into();
        self
    }

    pub fn port(mut self, port: u32) -> Self {
        self.port = port;
        self
    }

    pub fn port_auto_discover(mut self, discover: bool) -> Self {
        self.port_auto_discover = discover;
        self
    }

    pub fn thread_count(mut self, count: usize) -> Self {
        assert!(count != 0);
        self.thread_count = count;
        self
    }

    pub fn add_route(
        mut self,
        path: &str,
        method: RequestMethod,
        endpoint: EndpointMethod,
    ) -> Self {
        self.route_map
            .insert(path.to_string(), Endpoint::new(method, endpoint));
        self
    }

    pub fn build(mut self) -> Result<WebServer, String> {
        let listener: TcpListener;

        if self.port_auto_discover {
            loop {
                let l = TcpListener::bind(format!("{}:{}", self.addr, self.port));

                match l {
                    Ok(i) => {
                        listener = i;
                        break;
                    }
                    Err(e) => {
                        println!("Could not bind to port. Error {}. Trying next available", e);
                        self.port += 1;
                    }
                };
            }
        } else {
            match TcpListener::bind(format!("{}:{}", self.addr, self.port)) {
                Ok(i) => listener = i,
                Err(e) => {
                    return Err(format!(
                        "Failed to bind to {}:{}: {}",
                        self.addr, self.port, e
                    ));
                }
            }
        }

        let pool = ThreadPool::new(self.thread_count);

        println!("Creating server at address: {}:{}", self.addr, self.port);

        Ok(WebServer {
            addr: self.addr,
            port: self.port,
            pool,
            listener,
            route_map: self.route_map,
        })
    }
}

impl WebServer {
    pub fn new(addr: String, port: u32) -> WebServer {
        WebServerBuilder::new()
            .address(addr)
            .port(port)
            .build()
            .expect("Failed to create the web server")
    }

    pub fn start_server(&self) // Add result type?
    {
        for incoming in self.listener.incoming() {
            match incoming {
                Ok(connection) => {
                    println!("Received connection");
                    self.pool
                        .execute(move || WebServer::handle_connection(&connection));
                }
                Err(e) => println!("Failed to establish connection. Error {}", e),
            }
        }
    }

    fn handle_connection(mut stream: &TcpStream) {
        let mut buf_reader = BufReader::new(stream);

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

        println!("request {:#?}", http_request);
        println!("request body: {:?}", String::from_utf8_lossy(&request_body));

        let _parse_result = parse_request(&http_request, &request_body);

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        match stream.write_all(response.as_bytes()) {
            Ok(_) => println!("Responded"),
            Err(_) => print!("Failed to respond"),
        }
    }
}
