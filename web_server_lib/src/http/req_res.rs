use crate::http::error::HttpError;
use crate::http::req_res::request::Request;
use crate::http::req_res::response::Response;
use regex::Regex;
use std::collections::{HashMap, LinkedList};
use std::error::Error;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

pub mod internal_responses;
pub mod request;
pub mod response;

pub type RequestHandler = fn(&Request) -> Response;
pub type RequestMatcher = (Regex, Arc<RequestHandler>);

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum HttpVerb {
    Get,
    Put,
    Post,
    Patch,
    Delete,
    Option,
}

impl HttpVerb {
    pub fn from(val: &str) -> Result<HttpVerb, &'static str> {
        match val {
            "GET" => Ok(HttpVerb::Get),
            "PUT" => Ok(HttpVerb::Put),
            "PATCH" => Ok(HttpVerb::Patch),
            "POST" => Ok(HttpVerb::Post),
            "DELETE" => Ok(HttpVerb::Delete),
            "OPTION" => Ok(HttpVerb::Option),
            _ => Err("Failed to find matching verb"),
        }
    }
}

pub struct RequestMatcherMap {
    internal: Arc<HashMap<HttpVerb, Mutex<LinkedList<RequestMatcher>>>>,
}

impl RequestMatcherMap {
    pub fn add_matcher(
        &self,
        verb: HttpVerb,
        path_pattern: Regex,
        handler: RequestHandler,
    ) -> Result<(), Box<dyn Error + '_>> {
        if let Some(list) = self.internal.get(&verb) {
            list.lock()?.push_back((path_pattern, Arc::new(handler)));
            Ok(())
        } else {
            Err(Box::new(HttpError::new(format!("Invalid verb {:?}", verb))))
        }
    }

    pub fn match_request(&self, request: &Request) -> Option<Arc<RequestHandler>> {
        if let Some(mutex) = self.internal.get(&request.verb) {
            if let Ok(list) = mutex.lock() {
                for (re, handler) in list.iter() {
                    if re.is_match(&request.path) {
                        let arc = handler.clone();
                        return Some(arc);
                    }
                }
            }
        }
        None
    }

    pub fn new() -> RequestMatcherMap {
        let mut map: HashMap<HttpVerb, Mutex<LinkedList<RequestMatcher>>> = HashMap::new();
        map.insert(HttpVerb::Get, Mutex::new(LinkedList::new()));
        map.insert(HttpVerb::Patch, Mutex::new(LinkedList::new()));
        map.insert(HttpVerb::Post, Mutex::new(LinkedList::new()));
        map.insert(HttpVerb::Put, Mutex::new(LinkedList::new()));
        map.insert(HttpVerb::Delete, Mutex::new(LinkedList::new()));
        map.insert(HttpVerb::Option, Mutex::new(LinkedList::new()));

        RequestMatcherMap {
            internal: Arc::new(map),
        }
    }
}
