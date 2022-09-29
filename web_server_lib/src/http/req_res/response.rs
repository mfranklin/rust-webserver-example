use std::collections::LinkedList;

use regex::Regex;

pub struct Response {
    code: u16,
    msg: String,
    headers: LinkedList<String>,
    content: String,
    length: usize,
    binary: bool,
}

pub struct Kvp {
    pub key: String,
    pub value: String,
}

impl Response {
    pub fn get_code(&self) -> u16 {
        self.code
    }

    pub fn is_binary(&self) -> bool {
        self.binary
    }

    pub fn add_header(&mut self, name: &str, value: &str) -> () {
        self.headers.push_back(format!("{}: {}", name, value));
    }

    pub fn to_header_and_binary(&self) -> (String, Vec<u8>) {
        let (status_line, header_str) = self.build_header_string();
        let header = format!("{status_line}\r\n{header_str}\r\n");
        let contents = base64::decode(&self.content).unwrap();
        (header, contents)
    }

    pub fn build_binary(code: u16, contents: &[u8]) -> Response {
        Response {
            binary: true,
            code,
            msg: Self::get_msg_for_code(code),
            length: contents.len(),
            content: base64::encode(contents),
            headers: LinkedList::new(),
        }
    }

    pub fn build(code: u16, template: &str, replacement_keys: Vec<Kvp>) -> Response {
        let msg = Self::get_msg_for_code(code);
        let mut content = template.to_string();
        for kvp in replacement_keys {
            let re = Regex::new(&format!("#{}#", kvp.key)).unwrap();
            content = re.replace_all(&content, kvp.value).to_string();
        }

        let len = content.len();
        Response {
            binary: false,
            code,
            msg,
            content,
            length: len,
            headers: LinkedList::new(),
        }
    }

    fn build_header_string(&self) -> (String, String) {
        let status_line = format!("HTTP/1.1 {} {}", self.code, self.msg);
        let headers = &self.headers;

        let mut header_str = String::new();
        header_str.push_str(&format!("Content-Length: {}\r\n", self.length));
        let mut content_type_set = false;
        for header in headers {
            if header.contains("Content-Type") {
                content_type_set = true;
            }
            header_str.push_str(&format!("{}\r\n", header));
        }
        if !content_type_set {
            header_str.push_str(&format!("Content-Type: text/html\r\n"));
        }
        (status_line, header_str)
    }

    fn get_msg_for_code(code: u16) -> String {
        let msg = match code {
            200..=299 => "OK",
            300..=399 => "REDIRECT",
            401 => "NOT AUTHORIZED",
            403 => "FORBIDDEN",
            404 => "NOT FOUND",
            _ => "ERROR",
        };
        msg.to_string()
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let (status_line, header_str) = self.build_header_string();
        let content = &self.content;
        format!("{status_line}\r\n{header_str}\r\n{content}")
    }
}
