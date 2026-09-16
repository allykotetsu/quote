use anyhow::Error;
use owncast_plugin_sdk_rust::json_objects::status::Status;
use owncast_plugin_sdk_rust::prelude::OutgoingHttpResponse;
use serde::Deserialize;
use crate::helpers::remove_quotes;

#[derive(Deserialize)]
struct Request {
    pub(crate) ids: Vec<u32>,
}

pub(crate) fn function(body: &str) -> Result<OutgoingHttpResponse, Error> {
    // Deserialize JSON body to get quote IDs out.
    let Request { ids } = serde_json::from_str(body)?;

    // Attempt to remove quote.
    Ok(OutgoingHttpResponse::new(if remove_quotes(ids)? == 0 {
        // If 0 rows were modified, then return 404.
        Status::NotFound
    } else {
        // Else return 200.
        Status::Ok
    }))
}