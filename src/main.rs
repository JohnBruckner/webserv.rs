use minimal_web_server::server::server::WebServer;

fn main() {
    let server = WebServer::new("127.0.0.1".to_string(), 8888);

    server.start_server();
}
