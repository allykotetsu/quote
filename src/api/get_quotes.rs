use std::collections::HashMap;
use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::json_objects::status::Status;
use owncast_plugin_sdk_rust::prelude::OutgoingHttpResponse;
use serde::{Deserialize, Serialize};
use crate::helpers::{get_num_quotes, get_quotes};
use crate::objects::quote_object::Quote;

#[derive(Deserialize)]
struct Request {
    pub(crate) page: u32,
    pub(crate) count: u32
}

impl TryFrom<&HashMap<String, String>> for Request {
    type Error = Error;

    fn try_from(query: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let page = query.get("page").ok_or(anyhow!("Missing page"))?.parse()?;
        let count = query.get("count").ok_or(anyhow!("Missing page"))?.parse()?;
        Ok(Request { page, count })
    }
}

#[derive(Serialize)]
struct Response {
    pub(crate) quotes: HashMap<i64, String>,
    pub(crate) total: i64
}

pub(crate) fn function(query: &HashMap<String, String>) -> Result<OutgoingHttpResponse, Error> {
    let Request { page, count } = query.try_into()?;

    let page = page.max(1);
    let some_quotes = get_quotes(count, (page - 1) * count)?;
    let quotes: HashMap<i64, String> = some_quotes.iter().map(|Quote { id, message }| (*id, message.clone())).collect();
    let total = get_num_quotes()?;

    Ok(OutgoingHttpResponse::application_json(Status::Ok, Response { quotes, total })?)
}