use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{get_num_quotes, get_quote, get_server_name, random_quote};
use crate::objects::quote_object::Quote;

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    if let Some(id) = command_context.args.get(0) {
        // If an ID is supplied, then get its quote.
        let Ok(id) = id.parse::<u32>() else {
            // If parse failed tell user how to use command.
            return Ok(command_context.reply("`!quote` requires one whole, positive number when using the command this way.\n\n**Example**: `!quote 1`"));
        };

        if get_num_quotes()? > 0 {
            let Some(message) = get_quote(id)? else {
                // If a quote doesn't exist with that number, then tell user so.
                return Ok(command_context.reply("There isn't a quote with that number."));
            };

            // Format reply. Include streamer name if able to get.
            Ok(command_context.reply(&if let Some(name) = get_server_name() {
                format!("**Quote #{id}**:\n\n\"{message}\"\n\n*- {name}*")
            } else {
                format!("**Quote #{id}**:\n\n\"{message}\"")
            }))
        } else {
            // If there are no quotes then tell user how to make one.
            Ok(command_context.reply("There aren't any quotes yet! Create one by using `!newquote [insert quote here]`"))
        }
    } else {
        // Otherwise return a random one.
        let Some(Quote { id, message }) = random_quote()? else {
            // If no quotes exist then tell user.
            return Ok(command_context.reply("There aren't any quotes yet! Create one by using `!newquote [insert quote here]`"));
        };

        // Format reply. Include streamer name if able to get.
        Ok(command_context.reply(&if let Some(name) = get_server_name() {
            format!("**Quote #{id}**:\n\n\"{message}\"\n\n*- {name}*")
        } else {
            format!("**Quote #{id}**:\n\n\"{message}\"")
        }))
    }
}