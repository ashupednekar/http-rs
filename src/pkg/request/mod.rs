mod parser;
mod builder;

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub enum Method{
    GET,
    POST,
    PATCH,
    PUT,
    DELETE 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Body{
    Json(serde_json::Value),
    Bytes(Vec<u8>),
    Text(String)
}


#[derive(Debug, Clone)]
pub struct Request{
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: Body
}
