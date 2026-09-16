use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{remove_quote, UpdateQuoteResponse};

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    Ok(if let Some(id) = command_context.args.get(0) {
        match id.parse::<u32>() {
            // Attempt to remove quote.
            Ok(id) => match remove_quote(id)? {
                // If quote updated, then tell user it was removed.
                UpdateQuoteResponse::Updated => command_context.reply("Quote removed."),

                // Else tell the user no quote with that ID exists.
                UpdateQuoteResponse::NotFound => command_context.reply(&format!("No quote exists with the number {id}."))
            },

            // If int parse error, then tell user how to use command.
            Err(_) => command_context.reply("`!removequote` requires one whole, positive number for the quote.\n\n**Example**: `!removequote 1`")
        }
    } else {
        // If no arguments provided, then tell user to use some.
        command_context.reply("`!removequote` requires one whole, positive number for the quote.\n\n**Example**: `!removequote 1`")
    })
}