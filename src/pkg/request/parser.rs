use std::collections::HashMap;

use url::Url;

use super::{Body, Method, Request};

use crate::prelude::Result;

impl Request {
    pub fn parse(buf: Vec<u8>) -> Result<Self> {
        let sep = b"\r\n\r\n";
        let (method, path, headers, params, body) =
            if let Some(pos) = buf.windows(sep.len()).position(|window| window == sep) {
                let meta = String::from_utf8(buf[..pos].to_vec())?;
                let body = Body::new(buf[pos + 4..].to_vec());

                let mut parts = meta.splitn(2, "\r\n");
                let info = parts.next().ok_or("malformed http payload")?;

                let mut info_parts = info.trim().splitn(3, ' ');
                let method: Method = info_parts
                    .next()
                    .ok_or("missing HTTP method")?
                    .parse()
                    .map_err(|_| "invalid HTTP method")?;

                let path: String = info_parts.next().ok_or("missing HTTP path")?.to_string();

                let params: HashMap<String, String> =
                    Url::parse(&format!("http://dummy.host/{}", &path))?
                        .query_pairs()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();

                let headers: HashMap<String, String> = parts
                    .next()
                    .unwrap_or(": ")
                    .to_string()
                    .split("\r\n")
                    .filter_map(|s| {
                        let mut header = s.trim().splitn(2, ": ");
                        Some((
                            header.next()?.trim().to_string(),
                            header.next()?.trim().to_string(),
                        ))
                    })
                    .collect();

                (method, path, headers, params, body)
            } else {
                return Err("unterminated request buffer".into());
            };

        Ok(Self {
            method,
            path,
            headers,
            params,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_request_parsing() -> Result<()> {
        let raw_request = b"POST /api HTTP/1.1\r\n\
            Host: localhost:3000\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: 8\r\n\
            \r\n\
            hey jane";

        let request = Request::parse(raw_request.to_vec())?;
        tracing::info!("request: {:?}", &request);

        assert_eq!(request.method, Method::POST);
        assert_eq!(request.path, "/api");
        assert_eq!(request.headers.get("Content-Type").unwrap(), "text/plain");
        assert_eq!(request.headers.get("Content-Length").unwrap(), "8");
        assert_eq!(request.body, Body::Bytes(b"hey jane".to_vec()));

        Ok(())
    }
}
