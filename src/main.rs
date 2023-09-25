mod commands;
mod utils;

// ENV requirements
use std::env;
use dotenv::dotenv;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        application::{
            command::Command,
            interaction::{
                Interaction,
            }
        },
        gateway::Ready,
        prelude::*,
    },
    client::{
        Client,
        EventHandler,
        Context
    }
};
use songbird::SerenityInit;
use utils::interaction_message_response::interaction_message_response;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let _result = match command.data.name.as_str() {
                "ping" => commands::info::ping::run(&command, &ctx, &command.data.options).await,
                "info" => commands::info::info::run(&command, &ctx, &command.data.options).await,
                "pong" => commands::info::pong::run(&command, &ctx, &command.data.options).await,
                "join" => commands::music::join::run(&command, &ctx, &command.data.options).await,
                _ => {
                    interaction_message_response(&command, &ctx, "Command not implemented!").await;
                    "".to_string()
                },
            };
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let _ = match msg.content.as_str() {
            "~ping"  => commands::info::ping::message(ctx, msg).await,
            "~info"  => commands::info::info::message(ctx, msg).await,
            "~pong"  => commands::info::pong::message(ctx, msg).await,
            "~join"  => commands::music::join::message(ctx, msg).await,
            "~leave" => commands::music::leave::message(ctx, msg).await,
            _ => ()
        };
    }


    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

       let _commands =  Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::info::ping::register(command))
                .create_application_command(|command| commands::info::info::register(command))
                .create_application_command(|command| commands::info::pong::register(command))
                .create_application_command(|command| commands::music::join::register(command))
       }).await;

       println!("Registered command!");
    }

}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::non_privileged();

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
