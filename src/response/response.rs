use std::collections::HashMap;

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

