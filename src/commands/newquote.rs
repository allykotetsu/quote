use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{new_quote, NewQuoteResponse};

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    if command_context.args.is_empty() {
        Ok(command_context.reply("`!newquote` requires you to list the quote in question.\n\n**Example**: `!newquote Hi everyone.`"))
    } else {
        match new_quote(&command_context.arg_string)? {
            NewQuoteResponse::Duplicate => Ok(command_context.reply("This quote already exists!")),
            NewQuoteResponse::NewQuote(id) => Ok(command_context.reply(&format!("Quote #{id} added!")))
        }
    }
}