use serde::Deserialize;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

const HELP_MESSAGE: &str = "
Welcome to the help page! Here are some available commands:
- !ping: Responds with 'Pong!' to test if the bot is online.
- !cat: Generates a random cat image.
";
const HELP_COMMAND: &str = "!help";
const CAT_COMMAND: &str = "!cat";
const PING_COMMAND: &str = "!ping";
const PING_RESPONSE: &str = "Pong!";

#[derive(Debug, Deserialize)]
struct CatResponse {
    file: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == CAT_COMMAND {
            let response = reqwest::get("https://thecatapi.com/api/images/get?format=xml&type=jpg,png")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let cat: CatResponse = serde_xml_rs::from_str(&response).unwrap();
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(&cat.file).embed(|e| {
                    e.image(&cat.file);
                    e
                })
            }).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == PING_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, PING_RESPONSE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
