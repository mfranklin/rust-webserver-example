use crate::http::req_res::HttpVerb;

pub struct Request {
    pub verb: HttpVerb,
    pub path: String,
    pub headers: Vec<String>,
    pub body: String,
}
