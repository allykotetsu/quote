use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::update_quote;

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    if let Some(id) = command_context.args.get(0) {
        if command_context.args.len() > 1 {
            let quote = command_context.args[1..].join(" ");

            match id.parse::<u32>() {
                Ok(id) => match update_quote(id, &quote)? {
                    1 => Ok(command_context.reply("Quote updated.")),
                    0 => Ok(command_context.reply(&format!("No quote exists with the number {id}."))),
                    rows_affected => Err(anyhow!(format!("Something weird happened; {rows_affected} rows were affected."))),
                },
                Err(_) => Ok(command_context.reply("`!updatequote` requires two arguments: one whole, positive number for the quote id, and what to update the quote to.\n\n**Example**: `!updatequote 1 This is the new version of the quote!`"))
            }
        } else {
            Ok(command_context.reply("`!updatequote` requires two arguments: one whole, positive number for the quote id, and what to update the quote to.\n\n**Example**: `!updatequote 1 This is the new version of the quote!`"))
        }
    } else {
        Ok(command_context.reply("`!updatequote` requires two arguments: one whole, positive number for the quote id, and what to update the quote to.\n\n**Example**: `!updatequote 1 This is the new version of the quote!`"))
    }
}