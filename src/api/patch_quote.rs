use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::json_objects::status::Status;
use owncast_plugin_sdk_rust::prelude::OutgoingHttpResponse;
use serde::Deserialize;
use crate::helpers::{update_quote};

#[derive(Deserialize)]
struct Request {
    pub(crate) id: u32,
    pub(crate) message: String
}

pub(crate) fn function(body: &str) -> Result<OutgoingHttpResponse, Error> {
    let Request { id, message } = serde_json::from_str(body)?;

    match update_quote(id, &message)? {
        0 => Ok(OutgoingHttpResponse::new(Status::NotFound)),
        1 => Ok(OutgoingHttpResponse::new(Status::Ok)),
        x => Err(anyhow!(format!("Somehow {x} rows were modified.")))
    }
}