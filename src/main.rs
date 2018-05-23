//External Crates
#[macro_use] 
extern crate serenity;
extern crate typemap;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate url;

//Imports
use serenity::client::{ Client, Context};
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use std::env;

//Modules
mod commands;
mod trivia;
mod db;
mod question;
mod questionset;
mod optionset;


struct Handler;

//Event Handler for Discord Events
impl EventHandler for Handler {
    //Function to run whenever a message is received
    fn message(&self, ctx: Context, msg: Message) {
        let mut data = ctx.data.lock();
        let mut trivia_manager = data.get_mut::<trivia::TriviaManager>().unwrap();
        trivia_manager.on_message(msg);
    }
}


fn main() {

    // Login with a bot token from the environment
    let discord_token = &env::var("DISCORD_TOKEN").expect("token");
    let trivia_manager = trivia::TriviaManager::new();

    //Setup the bot client
    let mut client = Client::new(discord_token, Handler)
        .expect("Error creating client");

    //Store the trivia manager in our context's data map.
    {
        let mut data = client.data.lock();
        data.insert::<trivia::TriviaManager>(trivia_manager);
    }

    //Construct a client handler, which routes trivia commands to logic
    //This does not handle the answer input, only commands given with the prefix character
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
