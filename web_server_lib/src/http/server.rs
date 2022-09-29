use chrono::Utc;
use regex::Regex;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;

use crate::http::config::HttpConfig;
use crate::http::error::HttpError;
use crate::http::req_res::request::Request;
use crate::http::req_res::response::Response;
use crate::http::req_res::{internal_responses, HttpVerb, RequestHandler, RequestMatcherMap};
use crate::server::Server;
use crate::thread_pool::WorkerPool;

pub struct HttpServer {
    port: u16,
    thread_count: u8,
    handlers: Arc<RequestMatcherMap>,
    workers: WorkerPool,
}

impl HttpServer {
    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_thread_count(&self) -> u8 {
        self.thread_count
    }

    pub fn add_handler(
        &mut self,
        path_pattern: Regex,
        verb: HttpVerb,
        handler: RequestHandler,
    ) -> Result<(), Box<dyn Error + '_>> {
        self.handlers.add_matcher(verb, path_pattern, handler)
    }

    pub fn new(config: HttpConfig) -> HttpServer {
        let mut server = HttpServer {
            thread_count: config.thread_count,
            port: config.port,
            handlers: Arc::new(RequestMatcherMap::new()),
            workers: WorkerPool::new(config.thread_count as usize),
        };

        server
            .add_handler(
                Regex::new("/internal/css/index.css").unwrap(),
                HttpVerb::Get,
                |_| internal_responses::get_css(),
            )
            .unwrap();
        server
            .add_handler(
                Regex::new("/internal/img/img.png").unwrap(),
                HttpVerb::Get,
                |_| internal_responses::get_img(),
            )
            .unwrap();
        server
    }

    fn start_http_server(&self) -> Result<(), Box<dyn Error>> {
        println!(
            "Starting HTTP server with {} threads on port {}",
            self.thread_count, self.port
        );
        let listener = std::net::TcpListener::bind(("127.0.0.1", self.port))?;
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) -> () {
        let date = Utc::now();
        let map = self.handlers.clone();
        self.workers
            .execute(move || {
                let buf_reader = BufReader::new(&mut stream);
                let http_request: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();

                let request_info = Self::get_request_info(&http_request[0]);

                let response: Response = match request_info {
                    Err(e) => internal_responses::get_error(e),
                    Ok(info) => match Self::get_response(info.0, info.1, map) {
                        Some(r) => r,
                        None => internal_responses::get_not_found(),
                    },
                };

                Self::write_response(stream, &response).unwrap();

                let diff = Utc::now().time() - date.time();
                println!(
                    "{} '{}' {} {}",
                    date.to_string(),
                    http_request[0],
                    response.get_code(),
                    diff.num_milliseconds()
                )
            })
            .unwrap();
    }

    fn get_response(verb: HttpVerb, path: String, map: Arc<RequestMatcherMap>) -> Option<Response> {
        let request = Request {
            verb,
            path,
            headers: Vec::new(),
            body: "".to_string(),
        };

        if let Some(handler) = map.match_request(&request) {
            let response = handler(&request);
            return match response.get_code() {
                200..=299 => Some(response),
                404 => Some(internal_responses::get_not_found()),
                401 => Some(internal_responses::get_not_authorized()),
                _ => Some(internal_responses::get_error(Box::new(HttpError::new(
                    response.to_string(),
                )))),
            };
        }
        None
    }

    fn get_request_info(line: &str) -> Result<(HttpVerb, String), Box<dyn Error>> {
        let mut spliterator = line.split(" ");
        let verb = HttpVerb::from(spliterator.next().unwrap())?;
        let path = spliterator.next().unwrap();
        let version = spliterator.next().unwrap();
        if version != "HTTP/1.1" {
            return Err(Box::new(HttpError::new("Invalid HTTP version".to_string())));
        }
        Ok((verb, path.to_string()))
    }

    fn write_response(mut stream: TcpStream, response: &Response) -> Result<(), std::io::Error> {
        if response.is_binary() {
            let (header, contents) = response.to_header_and_binary();
            stream.write(header.as_bytes())?;
            Ok(stream.write_all(contents.as_slice())?)
        } else {
            Ok(stream.write_all(response.to_string().as_bytes())?)
        }
    }
}

impl Server for HttpServer {
    fn start(&self) -> Result<(), Box<dyn Error>> {
        self.start_http_server()?;
        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn Error>> {
        println!("Stopping HTTP server");
        Ok(())
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        self.stop().expect("HTTP Server failed to stop");
    }
}
