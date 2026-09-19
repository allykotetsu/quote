use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{update_quote, UpdateQuoteResponse};

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    let Some(id) = command_context.args.get(0) else {
        // If no args provided then tell user how to use command.
        return Ok(command_context.reply("`!updatequote` requires two arguments: one whole, positive number for the quote id, and what to update the quote to.\n\n**Example**: `!updatequote 1 This is the new version of the quote!`"));
    };

    if command_context.args.len() > 1 {
        // Take the remaining arguments (which are space separated) and join them with a space back into one string.
        let quote = command_context.args[1..].join(" ");

        let Ok(id) = id.parse::<u32>() else {
            // If error parsing then tell user how to use command.
            return Ok(command_context.reply("`!updatequote` requires two arguments: one whole, positive number for the quote id, and what to update the quote to.\n\n**Example**: `!updatequote 1 This is the new version of the quote!`"));
        };

        Ok(match update_quote(id, &quote)? {
            // If not found, then tell user there was no quote with that ID.
            UpdateQuoteResponse::NotFound => command_context.reply(&format!("No quote exists with the number {id}.")),

            // If successful, then tell user.
            UpdateQuoteResponse::Updated => command_context.reply("Quote updated.")
        })
    } else {
        // If only one argument then tell user how to use command.
        Ok(command_context.reply("`!updatequote` requires two arguments: one whole, positive number for the quote id, and what to update the quote to.\n\n**Example**: `!updatequote 1 This is the new version of the quote!`"))
    }
}