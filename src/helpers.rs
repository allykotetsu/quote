use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::json_objects::server_info::ServerInfo;
use owncast_plugin_sdk_rust::{owncast, run, sql_params};
use owncast_plugin_sdk_rust::json_objects::count::Count;
use owncast_plugin_sdk_rust::json_objects::sql_value::SqlValue;
use crate::objects::quote_id::QuoteId;
use crate::objects::quote_object::Quote;

pub(crate) fn get_server_name() -> Option<String> {
    match owncast::server::info() {
        Ok(ServerInfo { name, ..}) => name,
        Err(error) => {
            run!(owncast::log::error(&format!("{error:?}")));
            None
        }
    }
}

pub(crate) fn get_quotes(limit: u32, skip: u32) -> Result<Vec<Quote>, Error> {
    let sql_rows = owncast::sql::query("SELECT id, message FROM quote ORDER BY id ASC LIMIT ?, ?", sql_params![skip as i64, limit as i64])?;
    Ok(sql_rows.iter().filter_map(|sql_row| {
        if let Err(error) = sql_row {
            run!(owncast::log::error(&format!("{error:?}")))
        }
        sql_row.as_ref().ok()
    })
    .map(|quote: &Quote| quote.clone())
    .collect::<Vec<Quote>>())
}

pub(crate) fn get_num_quotes() -> Result<i64, Error> {
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

    match owncast::sql::exec("INSERT INTO quote (message) VALUES (?)", sql_params![message])?.rows_affected {
        0 => Ok(NewQuoteResponse::Duplicate),
        1 => {
            match owncast::sql::query_row("SELECT id FROM quote WHERE message=?", sql_params![message])? {
                Some(Ok(QuoteId { id })) => Ok(NewQuoteResponse::NewQuote(id)),
                Some(Err(error)) => Err(anyhow!(error)),
                None => Err(anyhow!("Something weird happened, couldn't get the id of the newly created quote."))
            }
        },
        x => Err(anyhow!(format!("Something weird happened, {x} rows were modified.")))
    }
}

pub(crate) fn update_quote(id: u32, message: &str) -> Result<i64, Error> {
    Ok(owncast::sql::exec("UPDATE quote SET message=? WHERE id=?", sql_params![message.trim(), id as i64])?.rows_affected)
}

pub(crate) fn remove_quote(id: u32) -> Result<i64, Error> {
    Ok(owncast::sql::exec("DELETE FROM quote WHERE id=?", sql_params![id as i64])?.rows_affected)
}

pub(crate) fn remove_quotes(ids: Vec<u32>) -> Result<i64, Error> {
    let string = "?, ".repeat(ids.len());
    let string = &string[..string.len() - 2];
    let ids: Vec<SqlValue> = ids.iter().map(|id| SqlValue::Number(*id as i64)).collect();
    Ok(owncast::sql::exec(&format!("DELETE FROM quote WHERE id IN ({string})"), ids)?.rows_affected)
}