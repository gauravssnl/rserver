use std::{collections::HashMap, io::BufRead};

const HOST: &str = "Host";

#[derive(Debug)]
pub enum Body {
    Empty,
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub url_raw: String,
    pub url: urlparse::Url,
    pub body: Body,
    pub raw_data: Vec<u8>,
    pub host: String,
    pub port: i32,
}

impl Request {
    pub fn new() -> Self {
        Request::default()
    }
}

impl Default for Request {
    fn default() -> Self {
        Request {
            method: String::default(),
            version: String::default(),
            headers: HashMap::default(),
            url_raw: String::default(),
            url: urlparse::Url::new(),
            body: Body::Empty,
            raw_data: Vec::default(),
            host: String::default(),
            port: 0,
        }
    }
}

impl From<Vec<u8>> for Request {
    fn from(buffer: Vec<u8>) -> Self {
        // let reader = buffer.reader();
        // let mut lines = reader.lines();
        let cursor = std::io::Cursor::new(&buffer);
        let mut lines_iter = cursor.lines().map(|l| l.unwrap());
        let first_line = lines_iter.next().unwrap();
        let first_line_vec: Vec<&str> = first_line.split(' ').collect();
        let method = first_line_vec.get(0).unwrap().to_string();
        let url_raw = first_line_vec.get(1).unwrap().to_string();
        let version = first_line_vec.get(2).unwrap().to_string();
        let headers = get_request_headers(&mut lines_iter);
        let body = Body::Empty;
        let url = urlparse::urlparse(&url_raw);
        let mut host: String = headers.get(HOST).unwrap().into();
        let mut port = if method == "CONNECT" { 443 } else { 80 };

        if host.contains(':') {
            let index = host.find(':').unwrap();
            host = host[..index].to_string();
            port = host[index + 1..].parse().unwrap(); //  skip colon (':') char
        }

        Request {
            method,
            url_raw,
            url,
            version,
            headers,
            body,
            raw_data: buffer,
            host,
            port,
        }
    }
}

pub fn get_request_headers(
    lines_iter: &mut impl Iterator<Item = String>,
) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    loop {
        match lines_iter.next() {
            None => break,
            Some(line) => {
                // println!("line: {:?}", line);
                if line.is_empty() {
                    break;
                }
                let header_line_vec: Vec<String> = line.split(':').map(|x| x.to_string()).collect();
                let header_key = header_line_vec.get(0).unwrap().to_string();
                let header_value = header_line_vec.get(1).unwrap().trim().to_string();
                headers.insert(header_key, header_value);
            }
        }
    }
    headers
}
