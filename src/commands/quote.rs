use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use owncast_plugin_sdk_rust::{owncast, sql_params};
use crate::helpers::get_server_name;
use crate::objects::quote_message::QuoteMessage;
use crate::objects::quote_object::Quote;

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    if let Some(id) = command_context.args.get(0) {
        match id.parse::<u32>() {
            Ok(id) => match owncast::sql::query_row("SELECT message FROM quote WHERE id=? LIMIT 1", sql_params![id as i64])? {
                Some(Ok(QuoteMessage { message })) => Ok(command_context.reply(
                    &if let Some(name) = get_server_name() {
                        format!("**Quote #{id}**:\n\n\"{message}\"\n\n*- {name}*")
                    } else {
                        format!("**Quote #{id}**:\n\n\"{message}\"")
                    }
                )),
                None => Ok(command_context.reply("There isn't a quote with that number.")),
                Some(Err(error)) => Err(anyhow!(error)),
            },
            Err(_) => Ok(command_context.reply("`!quote` requires one whole, positive number when using the command this way.\n\n**Example**: `!quote 1`"))
        }
    } else {
        match owncast::sql::query_row("SELECT id, message FROM quote ORDER BY RANDOM() LIMIT 1", vec![])? {
            Some(Ok(Quote { id, message })) => Ok(command_context.reply(&if let Some(name) = get_server_name() {
                format!("**Quote #{id}**:\n\n\"{message}\"\n\n*- {name}*")
            } else {
                format!("**Quote #{id}**:\n\n\"{message}\"")
            })),
            None => Ok(command_context.reply("There aren't any quotes yet! Create one by using `!newquote [insert quote here]`")),
            Some(Err(error)) => Err(anyhow!(error)),
        }
    }
}