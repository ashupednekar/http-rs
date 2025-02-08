use std::collections::HashMap;

use super::{Body, Method, Request};

use crate::prelude::Result;

impl Request{
    pub fn new(buf: Vec<u8>) -> Result<Self>{
        // accept bytes
        //if let Some(pos) = buffer.windows(separator.len()).position(|window| window ==
        //separator{} se get break off after headers for String stuff
        //explore performance considerations
        //tease simd
        //mention potential caveats
            
        let sep = b"\r\n\r\n";
        let (method, path, headers, body) = if let Some(pos) = buf.windows(sep.len()).position(|window| window == sep){
            let meta = String::from_utf8(buf[..pos].to_vec())?;
            let body = Body::Bytes(buf[pos+4..].to_vec());

            let mut parts = meta.splitn(2, "\r\n");
            let info = parts.next().ok_or("malformed http payload")?;
            
            let mut info_parts = info.trim().splitn(3, ' ');
            let method = info_parts
                .next()
                .ok_or("missing HTTP method")?
                .parse()
                .map_err(|_| "invalid HTTP method")?;
            
            let path = info_parts
                .next()
                .ok_or("missing HTTP path")?
                .to_string();

            let headers_str = parts
                .next()
                .unwrap_or(": ")
                .to_string();

            let headers: HashMap<String, String> = headers_str
                .split("\r\n")
                .filter_map(|s|{
                    let mut header = s.trim().splitn(2, ": ");
                    Some((
                        header.next()?.trim().to_string(),
                        header.next()?.trim().to_string()
                    ))
                })
                .collect();

            (method, path, headers, body)
        }else{
            return Err("unterminated request buffer".into());
        };

        let params = HashMap::new();
        Ok(Self{
            method, path, headers, params, body
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[tracing_test::traced_test]
    fn test_request_parsing() {
        let raw_request = b"POST /api HTTP/1.1\r\n\
            Host: localhost:3000\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: 8\r\n\
            \r\n\
            hey jane";
        
        let request = Request::new(raw_request.to_vec()).expect("Failed to parse request");
        tracing::info!("request: {:?}", &request);

        assert_eq!(request.method, Method::POST);
        assert_eq!(request.path, "/api");
        assert_eq!(request.headers.get("Content-Type").unwrap(), "text/plain");
        assert_eq!(request.headers.get("Content-Length").unwrap(), "8");
        assert_eq!(request.body, Body::Bytes(b"hey jane".to_vec()));
    }
}
