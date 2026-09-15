use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use crate::helpers::{get_num_quotes, get_quotes, get_server_name};
use crate::objects::quote_object::Quote;

pub(crate) fn function(command_context: &CommandContext) -> Result<(), Error> {
    match command_context.args.get(0).unwrap_or(&"1".to_string()).parse::<u32>() {
        Ok(page) => {
            let page = page.max(1);
            let quotes = get_quotes(10, (page - 1) * 10)?;

            Ok(match (quotes.len(), get_num_quotes()?) {
                (0, 0) => command_context.reply("There aren't any quotes yet! Create one by using `!newquote [insert quote here]`"),
                (0, total_count) => {
                    let pages = ((total_count as f64) / 10.).ceil();
                    command_context.reply(&format!("There aren't any quotes on this page. There are currently only {pages} pages of quotes."))
                },
                (_, total_count) => {
                    let pages = ((total_count as f64) / 10.).ceil();

                    let quotes = &quotes
                        .iter()
                        .map(|Quote { id, message }| format!("**#{id}**: \"{message}\""))
                        .collect::<Vec<String>>()
                        .join("\n\n");

                    let body = if let Some(name) = get_server_name() {
                        format!("*Quotes by {name}:*\n\n{quotes}\n\nPage **{page}** of {pages}")
                    } else {
                        format!("{quotes}\n\nPage **{page}** of {pages}")
                    };

                    command_context.reply(&body)
                }
            })
        },
        Err(_) => Ok(command_context.reply("`!quotes` requires one whole, positive number when using the command this way.\n\n**Example**: `!quote 1`"))
    }
}