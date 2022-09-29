use regex::Regex;
use std::time::Duration;
use std::{fs, thread};
use web_server_lib::http::config::HttpConfig;
use web_server_lib::http::req_res::HttpVerb;
use web_server_lib::http::req_res::request::Request;
use web_server_lib::http::req_res::response::Response;
use web_server_lib::http::server::{HttpServer};
use web_server_lib::server::Server;

fn main() {
    let config = HttpConfig {
        port: 8080,
        thread_count: 5,
    };
    let mut server = HttpServer::new(config);
    server
        .add_handler(Regex::new("/sleep(/)?.*").unwrap(), HttpVerb::Get, |_| {
            thread::sleep(Duration::from_secs(5));
            Response::build(200, "Slept for 5 seconds", Vec::new())
        })
        .unwrap();
    server
        .add_handler(Regex::new(r"/.*").unwrap(), HttpVerb::Get, handle_request)
        .unwrap();

    server.start().expect("Server failed to start");
}

fn handle_request(request: &Request) -> Response {
    println!("Looking for matching file {}", request.path);
    let mut path = "www".to_string();
    if request.path == "/" {
        path.push_str("/index.html");
    } else {
        path.push_str(&request.path);
    }
    let path_copy = path.clone();
    if let Ok(contents) = fs::read(path) {
        let mut response = Response::build_binary(200, contents.as_ref());
        if request.path.contains(".css") {
            response.add_header("Content-Type", "text/css");
        }
        if request.path.contains(".html") {
            response.add_header("Content-Type", "text/html");
        }
        if request.path.contains(".json") {
            response.add_header("Content-Type", "application/json");
        }
        if request.path.contains(".xml") {
            response.add_header("Content-Type", "application/xml");
        }
        if request.path.contains(".png") {
            response.add_header("Content-Type", "image/png");
        }
        response
    } else {
        println!("Failed to find file {}", path_copy);
        Response::build(404, "", Vec::new())
    }
}
