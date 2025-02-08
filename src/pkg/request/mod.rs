use std::collections::HashMap;

pub enum Method{
    GET,
    POST,
    PATCH,
    PUT,
    DELETE 
}

pub enum Body{
    Json(serde_json::Value),
    Bytes(Vec<u8>),
    Text(String)
}

pub struct Request{
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: Body
}
