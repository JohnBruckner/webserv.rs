use super::parse_error::HttpParseError;
use std::{collections::HashMap, hash::Hash};

static PROTOCOL: &str = "HTTP/1.1";

type ParseResult<T> = Result<T, Box<HttpParseError>>;

#[derive(Debug)]
enum RequestMethod {
    Get,
    Post,
    Options,
    NotImplemented,
}

#[derive(Debug)]
enum StatusCode {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    RequestTimeout = 408,
    InternalServerError = 500,
}

#[derive(Debug)]
struct RequestHead {
    method: RequestMethod,
    uri: String,
    protocol: String,
}

impl RequestHead {
    pub fn new(method: RequestMethod, uri: String, protocol: String) -> RequestHead {
        RequestHead {
            method: method,
            uri: uri,
            protocol: protocol,
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest<'a> {
    head: RequestHead,
    headers: HashMap<String, String>,
    body: &'a Vec<u8>,
}

impl<'a> HttpRequest<'a> {
    fn new(
        head: RequestHead,
        headers: HashMap<String, String>,
        body: &'a Vec<u8>,
    ) -> HttpRequest<'a> {
        HttpRequest {
            head: head,
            headers: headers,
            body: body,
        }
    }
}

#[derive(Debug)]
struct ResponseHead {
    protocol: String,
    code: StatusCode,
    message: String,
}

#[derive(Debug)]
struct HttpResponse {
    head: ResponseHead,
    headers: HashMap<String, String>,
    body: String,
}

pub fn parse_request<'a>(request: &Vec<String>, body: &'a Vec<u8>) -> ParseResult<HttpRequest<'a>> {
    if request.len() == 0 {
        return Err(Box::new(HttpParseError::new("Invalid http request")));
    }

    // first line of requst should be <METHOD> <URI> <PROTOCOL>
    let h: Vec<_> = request.first().unwrap().split(" ").collect();
    let method = match *h.first().unwrap() {
        "GET" => RequestMethod::Get,
        "POST" => RequestMethod::Post,
        "OPTIONS" => RequestMethod::Options,
        _ => RequestMethod::NotImplemented,
    };
    let req_head = RequestHead::new(
        method,
        h.iter().nth(1).unwrap().to_string(),
        h.iter().nth(2).unwrap().to_string(),
    );
    // headers are of pattern Header-name: header-value (Content-Type: application/json)
    let headers: HashMap<String, String> = request
        .iter()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();

            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect();

    println!("Request head: {:#?}", req_head);
    println!("Request headers: {:#?}", headers);

    Ok(HttpRequest::new(req_head, headers, body))
}
