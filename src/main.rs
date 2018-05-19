#[macro_use] 
extern crate serenity;
extern crate typemap;

use serenity::client::{ Client, Context};
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.lock();
        let trivia_manager = data.get::<trivia::TriviaManager>().unwrap();
        trivia_manager.on_message(&msg);
    }
}

mod commands;
mod trivia;

fn main() {
    // Login with a bot token from the environment
    let discord_token = &env::var("DISCORD_TOKEN").expect("token");
    let trivia_manager = trivia::TriviaManager::new();

    //Setup the bot client
    let mut client = Client::new(discord_token, Handler)
        .expect("Error creating client");

    {
        //Get context data for inserting trivia manager struct
        let mut data = client.data.lock();
        data.insert::<trivia::TriviaManager>(trivia_manager);
    }
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(".")) // set the bot's prefix to "."
        .command("trivia", |c| c
             .cmd(commands::trivia::trivia_handler))
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
