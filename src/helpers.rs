use std::collections::HashMap;
use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::json_objects::server_info::ServerInfo;
use owncast_plugin_sdk_rust::{owncast, sql_params};
use owncast_plugin_sdk_rust::helpers::run;
use owncast_plugin_sdk_rust::json_objects::count::Count;
use owncast_plugin_sdk_rust::json_objects::sql_value::SqlValue;
use crate::objects::quote_id::QuoteId;
use crate::objects::quote_message::QuoteMessage;
use crate::objects::quote_object::Quote;

pub(crate) fn get_server_name() -> Option<String> {
    match owncast::server::info() {
        // If server info is returned successfully, then return the optional streamer name.
        Ok(ServerInfo { name, .. }) => name,

        // If there is an error getting back the server info, then report and return None.
        Err(error) => {
            run(owncast::log::error(&format!("{error:?}")));
            None
        }
    }
}

pub(crate) fn get_quotes(limit: u32, skip: u32) -> Result<HashMap<u32, String>, Error> {
    // Select quotes from the SQL database pertaining to the pagination arguments (limit and skip).
    let sql_rows = owncast::sql::query("SELECT id, message FROM quote ORDER BY id ASC LIMIT ?, ?", sql_params![skip as i64, limit as i64])?;

    Ok(sql_rows.iter().filter_map(|sql_row| {
        // Filter map on each SQL row. If SQL row is an error then log.
        if let Err(error) = sql_row {
            run(owncast::log::error(&format!("{error:?}")))
        }

        // Take the SQL Row, convert it to a reference, and filter out Errs / convert Ok(x) to x.
        sql_row.as_ref().ok()
    })

    // Clone (dereference) each quote.
    .map(|Quote { id, message }: &Quote| (*id as u32, message.clone()))

    // If no errors have occurred, then collect into Vec and return Ok.
    .collect())
}

pub(crate) fn get_num_quotes() -> Result<i64, Error> {
    // Get number of quotes. If None is returned then return 0.
    if let Some(count) = owncast::sql::query_row::<Count>("SELECT COUNT(*) FROM quote", vec![])? {
        Ok(count?.count)
    } else {
        Ok(0)
    }
}

pub(crate) enum NewQuoteResponse {
    Duplicate,
    NewQuote(i64)
}

pub(crate) fn new_quote(message: &str) -> Result<NewQuoteResponse, Error> {
    let message = message.trim();

    // Insert the trimmed quote into the database and return back how many rows were modified if no errors occurred.
    match owncast::sql::exec("INSERT INTO quote (message) VALUES (?)", sql_params![message])?.rows_affected {
        // If no rows were modified, then there was already a quote with that message.
        0 => Ok(NewQuoteResponse::Duplicate),

        // If one row was modified, then the new quote was inserted successfully.
        1 => {
            // Get back the generated ID of the newly created quote.
            match owncast::sql::query_row("SELECT id FROM quote WHERE message=?", sql_params![message])? {
                // If an object was returned and the object is Ok, then return the newly created ID.
                Some(Ok(QuoteId { id })) => Ok(NewQuoteResponse::NewQuote(id)),

                // If an error is present, then error.
                Some(Err(error)) => Err(anyhow!(error)),

                // If SQL returned nothing, then something wrong happened on the host's end.
                None => Err(anyhow!("Something weird happened, couldn't get the id of the newly created quote."))
            }
        },

        // If any number of rows besides 0 or 1 were modified, then something wrong happened on the host's end.
        x => Err(anyhow!(format!("Something weird happened, {x} rows were modified.")))
    }
}

pub(crate) enum UpdateQuoteResponse {
    NotFound,
    Updated
}

pub(crate) fn update_quote(id: u32, message: &str) -> Result<UpdateQuoteResponse, Error> {
    // Update the quote and return back how many rows were modified.
    match owncast::sql::exec("UPDATE quote SET message=? WHERE id=?", sql_params![message.trim(), id as i64])?.rows_affected {
        // If no rows were modified, then no quote exists with that ID.
        0 => Ok(UpdateQuoteResponse::NotFound),

        // If one row was modified, then the quote was successfully updated.
        1 => Ok(UpdateQuoteResponse::Updated),

        // If any number of rows besides 0 or 1 were modified, then something wrong happened on the host's end.
        x => Err(anyhow!(format!("Something weird happened, {x} rows were modified.")))
    }
}

pub(crate) fn remove_quote(id: u32) -> Result<UpdateQuoteResponse, Error> {
    // Remove the quote and return back how many rows were modified.
    match owncast::sql::exec("DELETE FROM quote WHERE id=?", sql_params![id as i64])?.rows_affected {
        // If no rows were modified, then no quote exists with that ID.
        0 => Ok(UpdateQuoteResponse::NotFound),

        // If one row was modified, then the quote was successfully removed.
        1 => Ok(UpdateQuoteResponse::Updated),

        // If any number of rows besides 0 or 1 were modified, then something wrong happened on the host's end.
        x => Err(anyhow!(format!("Something weird happened, {x} rows were modified.")))
    }
}

pub(crate) fn remove_quotes(ids: Vec<u32>) -> Result<i64, Error> {
    // Start constructing query string. Write a ? for each id present.
    let string = "?, ".repeat(ids.len());

    // Remove the final ", "
    let string = &string[..string.len() - 2];

    // Take each ID and map it to a SqlValue::Number for use as the SQL parameters.
    let ids: Vec<SqlValue> = ids.iter().map(|id| SqlValue::Integer(*id as i64)).collect();

    // Execute SQL delete for quotes if the quote ID is in the ID list.
    Ok(owncast::sql::exec(&format!("DELETE FROM quote WHERE id IN ({string})"), ids)?.rows_affected)
}

pub(crate) fn random_quote() -> Result<Option<Quote>, Error> {
    // Get random quote and extract message and ID if there are no errors.
    Ok(if let Some(quote) = owncast::sql::query_row("SELECT id, message FROM quote ORDER BY RANDOM() LIMIT 1", vec![])? {
        Some(quote?)
    } else {
        None
    })
}

pub(crate) fn get_quote(id: u32) -> Result<Option<String>, Error> {
    // Get quote matching ID and extract quote text if there are no errors.
    Ok(if let Some(quote) = owncast::sql::query_row::<QuoteMessage>("SELECT message FROM quote WHERE id=? LIMIT 1", sql_params![id as i64])? {
        Some(quote?.message)
    } else {
        None
    })
}