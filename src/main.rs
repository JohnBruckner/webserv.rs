use minimal_web_server::http::parser::RequestMethod;
use minimal_web_server::server::server::{WebServer, WebServerBuilder};

fn main() {
    let server = WebServerBuilder::new()
        .address("127.0.0.1".to_string())
        .port(8888)
        .port_auto_discover(true)
        .build()
        .unwrap()
        .start_server();
    //     .add_route(
    //     "/",
    //     RequestMethod::Get,
    //     hello_world,
    // )
}

pub fn hello_world() -> String {
    "Hello World".to_string()
}
