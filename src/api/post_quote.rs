use anyhow::Error;
use owncast_plugin_sdk_rust::json_objects::status::Status;
use owncast_plugin_sdk_rust::prelude::OutgoingHttpResponse;
use serde::{Deserialize, Serialize};
use crate::helpers::{new_quote, NewQuoteResponse};

#[derive(Deserialize)]
struct Request {
    pub(crate) message: String
}

#[derive(Serialize)]
struct Response {
    pub(crate) id: i64
}

pub(crate) fn function(body: &str) -> Result<OutgoingHttpResponse, Error> {
    let Request { message } = serde_json::from_str(body)?;

    Ok(match new_quote(&message)? {
        NewQuoteResponse::Duplicate => OutgoingHttpResponse::new(Status::BadRequest),
        NewQuoteResponse::NewQuote(id) => OutgoingHttpResponse::application_json(Status::Created, Response { id })?
    })
}