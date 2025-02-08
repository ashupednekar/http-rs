use super::{Response, StatusCode};
use std::collections::HashMap;

impl Response {
    pub fn new(body: Vec<u8>, status: StatusCode) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), format!("{}", body.len()));
        Self {
            body,
            headers,
            status,
        }
    }

    pub fn set_header(&mut self, key: &str, val: &str) {
        self.headers.insert(key.to_string(), val.to_string());
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: String = self.status.as_http();
        for (k, v) in self.headers.iter() {
            res += &format!("{}: {}\r\n", &k, &v);
        }
        res += "\r\n";
        let mut res: Vec<u8> = res.into_bytes();
        res.extend_from_slice(&self.body);
        res
    }
}
