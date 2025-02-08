mod parser;

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    GET,
    POST,
    PATCH,
    PUT,
    DELETE,
}

impl FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    Json(serde_json::Value),
    Bytes(Vec<u8>),
    Text(String),
}

impl Body{
    pub fn new(buf: Vec<u8>) -> Self{
        match serde_json::from_slice(&buf){
            Ok(v) => Body::Json(v),
            Err(_) => {
                match String::from_utf8(buf.clone()){
                    Ok(s) => Body::Text(s),
                    Err(_) => Body::Bytes(buf)
                }
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: Body,
}
