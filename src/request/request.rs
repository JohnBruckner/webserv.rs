use crate::http::parse_error::HttpParseError;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Options,
    NotImplemented,
}
#[derive(Debug)]
pub struct RequestHead {
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
    pub(crate) fn new(
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

    pub fn content_type(&self) -> Option<&String> {
        self.headers.get("Content-Type")
    }

    pub fn body_to_json<T: serde::de::DeserializeOwned>(&self) -> Result<T, Box<dyn Error>> {
        if let Some(content_type) = self.content_type() {
            if content_type.starts_with("application/json") {
                let body_str = std::str::from_utf8(&self.body)?;
                let parsed: T = serde_json::from_str(body_str)?;
                return Ok(parsed);
            }
        }
        Err(Box::new(HttpParseError::new(
            "Content is not application/json",
        )))
    }

    pub fn body_to_text(&self) -> Result<String, Box<dyn Error>> {
        if let Some(content_type) = self.content_type() {
            let content_type = content_type.to_lowercase();
            if content_type.starts_with("text/")
                || content_type.starts_with("application/json")
                || content_type.starts_with("application/xml")
                || content_type.contains("+json")
                || content_type.contains("+xml")
                || content_type.contains("+text")
            {
                let body_str = std::str::from_utf8(&self.body)?;
                return Ok(body_str.to_string());
            }
            return Err(Box::new(HttpParseError::new("Content type is not text")));
        }
        Err(Box::new(HttpParseError::new(
            "request body has not content-type",
        )))
    }
}
