use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{new_quote, NewQuoteResponse};

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    Ok(if command_context.args.is_empty() {
        // If no arguments provided, then tell user to use some.
        command_context.reply("`!newquote` requires you to list the quote in question.\n\n**Example**: `!newquote Hi everyone.`")
    } else {
        // Attempt to create new quote.
        if let NewQuoteResponse::NewQuote(id) = new_quote(&command_context.arg_string)? {
            // If a new quote then tell user the quote's ID.
            command_context.reply(&format!("Quote #{id} added!"));
        } else {
            // If duplicate quote then tell user this quote already exists.
            command_context.reply("This quote already exists!")
        }
    })
}