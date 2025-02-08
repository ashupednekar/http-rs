use std::collections::HashMap;

use super::{Body, Method, Request};

use crate::prelude::Result;

impl Request{
    pub fn new(data: Vec<u8>) -> Result<Self>{
        // accept bytes
        //if let Some(pos) = buffer.windows(separator.len()).position(|window| window ==
        //separator{} se get break off after headers for String stuff
        //explore performance considerations
        //tease simd
        //mention potential caveats

        let method = Method::POST;
        let path = "/api".to_string();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        headers.insert("Content-Length".to_string(), "8".to_string());
        let params = HashMap::new();
        let body = Body::Bytes("hey jane".as_bytes().to_vec());
        Ok(Self{
            method, path, headers, params, body
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request_parsing() {
        let raw_request = b"POST /api HTTP/1.1\r\n\
            Host: localhost:3000\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: 8\r\n\
            \r\n\
            hey jane";
        
        let request = Request::new(raw_request.to_vec()).expect("Failed to parse request");

        assert_eq!(request.method, Method::POST);
        assert_eq!(request.path, "/api");
        assert_eq!(request.headers.get("Content-Type").unwrap(), "text/plain");
        assert_eq!(request.headers.get("Content-Length").unwrap(), "8");
        assert_eq!(request.body, Body::Bytes(b"hey jane".to_vec()));
    }
}
