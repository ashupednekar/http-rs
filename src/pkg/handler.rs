use serde_json::json;

use crate::{prelude::Result, pkg::response::StatusCode};

use super::{request::Request, response::Response};



pub fn handle(req: Request) -> Result<Response>{
    tracing::info!("received req: {:?}", &req);
    let res = Response::new(serde_json::to_vec(&json!({"msg": "success"}))?, StatusCode::Ok);
    Ok(res) 
}
