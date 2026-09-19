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
    // Deserialize JSON body to get quote text out.
    let Request { message } = serde_json::from_str(body)?;

    // Attempt to create new quote.
    Ok(if let NewQuoteResponse::NewQuote(id) = new_quote(&message)? {
        // Else return back 201 with the new quote's ID.
        OutgoingHttpResponse::application_json(Status::Created, Response { id })?
    } else {
        // If quote already exists, then return 400.
        OutgoingHttpResponse::new(Status::BadRequest)
    })
}