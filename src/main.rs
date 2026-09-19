mod helpers;
mod objects;
mod api;
mod commands;

use anyhow::Error;
use owncast_plugin_sdk_rust::command::command_builder::CommandBuilder;
use owncast_plugin_sdk_rust::prelude::*;
use owncast_plugin_sdk_rust::owncast;
use owncast_plugin_sdk_rust::command::command_context::CommandContext;
use owncast_plugin_sdk_rust::helpers::run;
use owncast_plugin_sdk_rust::json_objects::method::Method;
use owncast_plugin_sdk_rust::json_objects::partial_incoming_http_request::PartialIncomingHttpRequest;
use owncast_plugin_sdk_rust::json_objects::status::Status;
use crate::api::{delete_quotes, get_quotes, patch_quote, post_quote};
use crate::commands::{newquote, quote, quotes, removequote, updatequote};

fn run_and_report_error(function: fn(&CommandContext) -> Result<(), Error>, command_context: &CommandContext) {
    if let Err(error) = function(command_context) {
        command_context.reply("There was an internal server error running this command.");
        run(owncast::log::error(&format!("{error:?}")));
    }
}

fn http_500_if_error(error: Error) -> OutgoingHttpResponse {
    run(owncast::log::error(&format!("{error:?}")));
    OutgoingHttpResponse::new(Status::InternalServerError)
}

define_plugin!(|mut plugin_builder| {
    // Init database with table info.
    plugin_builder.on_init(|_| {
        run(owncast::sql::exec(r#"CREATE TABLE IF NOT EXISTS quote (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            message TEXT NOT NULL UNIQUE
        )"#, vec![]));
    })?;

    plugin_builder.commands("!", false, vec![
        // Command for getting back a quote.
        CommandBuilder::new("quote", |_, command_context| {
            run_and_report_error(quote::function, command_context);
        })
        .with_description("Get a quote, if number is omitted then a random one is gotten.")
        .with_usage("!quote [quote number, or omit for random]")
        .with_cooldown(1000),

        // Command for registering a new quote.
        CommandBuilder::new("newquote", |_, command_context| {
            run_and_report_error(newquote::function, command_context);
        })
        .with_description("Create a new quote.")
        .with_usage("!newquote [insert quote here]")
        .with_cooldown(1000),

        // Command for getting back every quote
        CommandBuilder::new("quotes", |_, command_context| {
            run_and_report_error(quotes::function, command_context);
        })
        .with_description("Get back the list of quotes, paginated.")
        .with_usage("!quotes [page]")
        .with_cooldown(1000),

        // Command for removing a quote. Mod only.
        CommandBuilder::new("removequote", |_, command_context| {
            run_and_report_error(removequote::function, command_context);
        })
        .with_description("Remove a given quote.")
        .with_usage("!removequote [quote number]")
        .mod_only(),

        // Command for updating a quote. Mod only.
        CommandBuilder::new("updatequote", |_, command_context| {
            run_and_report_error(updatequote::function, command_context);
        })
        .with_description("Update a given quote.")
        .with_usage("!updatequote [quote number] [new quote]")
        .mod_only()
    ])?;

    // Endpoint for getting list of quotes.
    plugin_builder.on_http_request(Method::Get, "/admin/api/quotes", |_, PartialIncomingHttpRequest { query, .. }| {
        get_quotes::function(query).unwrap_or_else(http_500_if_error)
    })?;

    // Endpoint for making a new post.
    plugin_builder.on_http_request(Method::Post, "/admin/api/quote", |_, PartialIncomingHttpRequest { body, .. }| {
        post_quote::function(&body).unwrap_or_else(http_500_if_error)
    })?;

    // Endpoint for removing a quote.
    plugin_builder.on_http_request(Method::Delete, "/admin/api/quotes", |_, PartialIncomingHttpRequest { body, .. }| {
        delete_quotes::function(&body).unwrap_or_else(http_500_if_error)
    })?;

    // Endpoint for updating a quote.
    plugin_builder.on_http_request(Method::Patch, "/admin/api/quote", |_, PartialIncomingHttpRequest { body, .. }| {
        patch_quote::function(&body).unwrap_or_else(http_500_if_error)
    })?;

    Ok(plugin_builder)
});

fn main() {

}