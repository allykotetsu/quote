use anyhow::Error;
use owncast_plugin_sdk_rust::json_objects::status::Status;
use owncast_plugin_sdk_rust::prelude::OutgoingHttpResponse;
use serde::Deserialize;
use crate::helpers::{update_quote, UpdateQuoteResponse};

#[derive(Deserialize)]
struct Request {
    pub(crate) id: u32,
    pub(crate) message: String
}

pub(crate) fn function(body: &str) -> Result<OutgoingHttpResponse, Error> {
    // Deserialize JSON body to get quote text and ID out.
    let Request { id, message } = serde_json::from_str(body)?;

    // Attempt to update quote.
    Ok(OutgoingHttpResponse::new(match update_quote(id, &message)? {
        // This is pretty self explanatory.
        UpdateQuoteResponse::NotFound => Status::NotFound,
        UpdateQuoteResponse::Updated => Status::Ok
    }))
}