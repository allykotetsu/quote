# Quote

Quote is a simple plugin that adds the ability for users to register quotes that the streamer has said, and then call them back to chat later.

## Creating a new quote

To create a new quote, use the `!newquote` command. Everything following the space after the command name will be registered as part of the quote. For example, `!newquote Hi everyone!!!` will register the quote "Hi everyone!!!". After you create the quote, you will get back the number of the quote so you can display it back later.

Admins can create new quotes through the admin panel under Plugins > Quote.

## Finding a quote

To find a quote, use the `!quotes` command; this will print out every quote registered, along with their associated numbers. If you have more than 10 quotes, then the results will be paginated, and you can supply the page with the command. For example, `!quotes 3` will list the third page of quotes. (Which is likely every quote with a number between 21 and 30.)

Admins can view the list of every quote through the admin panel under Plugins > Quote.

## Displaying a quote

To display a quote, use the `!quote` command. When a number is provided, then the quote with the matching number will be displayed. If a number is not provided, then a random quote will be displayed.

## Removing a quote

Moderators can remove quotes with the `!removequote` command. The number for the quote in question must be provided.

Admins can remove quotes through the admin panel under Plugins > Quote.

## Updating a quote

Moderators can update quotes with the `!updatequote` command. The number for the quote in question must be provided, along with the new text of the quote.

Admins can remove quotes through the admin panel under Plugins > Quote.