use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{get_num_quotes, get_quote, get_server_name, random_quote};
use crate::objects::quote_object::Quote;

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    Ok(if let Some(id) = command_context.args.get(0) {
        // If an ID is supplied, then get its quote.
        match id.parse::<u32>() {
            Ok(id) => if get_num_quotes()? > 0 {
                if let Some(message) = get_quote(id)? {
                    // Format reply. Include streamer name if able to get.
                    command_context.reply(&if let Some(name) = get_server_name() {
                        format!("**Quote #{id}**:\n\n\"{message}\"\n\n*- {name}*")
                    } else {
                        format!("**Quote #{id}**:\n\n\"{message}\"")
                    })
                } else {
                    command_context.reply("There isn't a quote with that number.")
                }
            } else {
                command_context.reply("There aren't any quotes yet! Create one by using `!newquote [insert quote here]`")
            }

            // If parse failed tell user how to use command.
            Err(_) => command_context.reply("`!quote` requires one whole, positive number when using the command this way.\n\n**Example**: `!quote 1`")
        }
    } else {
        // Otherwise return a random one.
        if let Some(Quote { id, message }) = random_quote()? {
            // Format reply. Include streamer name if able to get.
            command_context.reply(&if let Some(name) = get_server_name() {
                format!("**Quote #{id}**:\n\n\"{message}\"\n\n*- {name}*")
            } else {
                format!("**Quote #{id}**:\n\n\"{message}\"")
            })
        } else {
            // If no quotes exist then tell user.
            command_context.reply("There aren't any quotes yet! Create one by using `!newquote [insert quote here]`")
        }
    })
}