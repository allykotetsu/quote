use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{get_num_quotes, get_quotes, get_server_name};

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    // Get page or 1.
    Ok(match command_context.args.get(0).unwrap_or(&"1".to_string()).parse::<u32>() {
        Ok(page) => {
            // Get page (1-indexed) and quotes for that page.
            let page = page.max(1);
            let quotes = get_quotes(10, (page - 1) * 10)?;

            match (quotes.len(), get_num_quotes()?) {
                // If there are no quotes at all, then tell user so.
                (_, 0) => command_context.reply("There aren't any quotes yet! Create one by using `!newquote [insert quote here]`"),

                // If there are no quotes on page, but there are not 0 quotes total, then tell user there aren't quotes on this page.
                (0, total_count) => {
                    let pages = ((total_count as f64) / 10.).ceil();
                    let word1 = if pages <= 1. { "is" } else { "are" };
                    let word2 = if pages <= 1. { "page" } else { "pages" };
                    command_context.reply(&format!("There aren't any quotes on that page. There {word1} currently only {pages} {word2} of quotes."))
                },

                // If there are not 0 quotes on the page, then give user quotes.
                (_, total_count) => {
                    let pages = ((total_count as f64) / 10.).ceil();

                    // Turn each quote into a formatted String for the chat message.
                    let quotes = &quotes
                        .iter()
                        .map(|(id, message)| format!("> **#{id}**: \"{message}\""))
                        .collect::<Vec<String>>()
                        .join("\n\n");

                    // Format quotes block into chat message. Include streamer name if able to retrieve it.
                    let body = if let Some(name) = get_server_name() {
                        format!("*{total_count} quotes by {name}:*\n\n{quotes}\n\nPage **{page}** of {pages}")
                    } else {
                        format!("*{total_count} quotes:*\n\n*{quotes}\n\nPage **{page}** of {pages}")
                    };

                    command_context.reply(&body)
                }
            }
        },

        // If there was an error parsing page, then tell user how to use command.
        Err(_) => command_context.reply("`!quotes` requires one whole, positive number when using the command this way.\n\n**Example**: `!quotes 1`")
    })
}