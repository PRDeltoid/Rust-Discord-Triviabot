#[macro_use] extern crate serenity;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler {}


fn main() {
    // Login with a bot token from the environment
    let discord_token = &env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::new(discord_token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .cmd("ping", ping));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
    
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});
