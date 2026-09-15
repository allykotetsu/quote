use anyhow::{anyhow, Error};
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::remove_quote;

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    if let Some(id) = command_context.args.get(0) {
        match id.parse::<u32>() {
            Ok(id) => match remove_quote(id)? {
                1 => Ok(command_context.reply("Quote removed.")),
                0 => Ok(command_context.reply(&format!("No quote exists with the number {id}."))),
                rows_affected => Err(anyhow!(format!("Something weird happened; {rows_affected} rows were affected."))),
            },
            Err(_) => Ok(command_context.reply("`!removequote` requires one whole, positive number for the quote.\n\n**Example**: `!removequote 1`"))
        }
    } else {
        Ok(command_context.reply("`!removequote` requires one whole, positive number for the quote.\n\n**Example**: `!removequote 1`"))
    }
}