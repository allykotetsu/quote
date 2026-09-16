use std::collections::HashMap;
use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::json_objects::status::Status;
use owncast_plugin_sdk_rust::prelude::OutgoingHttpResponse;
use serde::{Deserialize, Serialize};
use crate::helpers::{get_num_quotes, get_quotes};

#[derive(Deserialize)]
struct Request {
    pub(crate) page: u32,
    pub(crate) count: u32
}

impl TryFrom<&HashMap<String, String>> for Request {
    type Error = Error;

    fn try_from(query: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let page = query.get("page").ok_or(anyhow!("Missing page"))?.parse()?;
        let count = query.get("count").ok_or(anyhow!("Missing count"))?.parse()?;
        Ok(Request { page, count })
    }
}

#[derive(Serialize)]
struct Response {
    pub(crate) quotes: HashMap<u32, String>,
    pub(crate) total: i64
}

pub(crate) fn function(query: &HashMap<String, String>) -> Result<OutgoingHttpResponse, Error> {
    // Deserialize query parameters to get pagination info.
    let Request { page, count } = query.try_into()?;

    let page = page.max(1);
    let quotes = get_quotes(count, (page - 1) * count)?;

    // Return 200 with page of quotes and amount of total quotes.
    let total = get_num_quotes()?;
    Ok(OutgoingHttpResponse::application_json(Status::Ok, Response { quotes, total })?)
}