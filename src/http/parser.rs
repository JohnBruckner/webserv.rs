use super::parse_error::HttpParseError;
use std::{collections::HashMap, error::Error};
use crate::request::request::{HttpRequest, RequestHead, RequestMethod};

static PROTOCOL: &str = "HTTP/1.1";

type ParseResult<T> = Result<T, Box<HttpParseError>>;

pub fn parse_request<'a>(request: &Vec<String>, body: &'a Vec<u8>) -> ParseResult<HttpRequest<'a>> {
    if request.len() == 0 {
        return Err(Box::new(HttpParseError::new("Invalid http request")));
    }

    // first line of request should be <METHOD> <URI> <PROTOCOL>
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

    println!("request head: {:#?}", req_head);
    println!("request headers: {:#?}", headers);

    Ok(HttpRequest::new(req_head, headers, body))
}
