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
        let method = Method::GET;
        let path = "/".to_string();
        let headers = HashMap::new();
        let params = HashMap::new();
        let body = Body::Bytes(vec![]);
        Ok(Self{
            method, path, headers, params, body
        })
    }
}
