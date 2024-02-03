extern crate dotenv;
use commands::COMMANDS_GROUP;
use dotenv::dotenv;
use event_handler::Handler;
use songbird::SerenityInit;
use std::{collections::HashSet, env};

use serenity::{
    all::Http,
    framework::{standard::Configuration, StandardFramework},
    prelude::*,
};
pub mod commands;
pub mod event_handler;
pub mod sound;
pub mod util;
pub mod voice_event_handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info {:?}", why),
    };

    let framework = StandardFramework::new().group(&COMMANDS_GROUP);
    framework.configure(Configuration::new().owners(owners).prefix("v!"));

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
