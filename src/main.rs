mod commands;

// ENV requirements
use std::env;
use dotenv::dotenv;

// Serenity
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            let _result = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command, &ctx, &command.data.options).await,
                _ => {
                    async {
                        if let Err(why) = command
                            .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content("not implemented :(".to_string()))
                        }).await
                        {
                        println!("Cannot respond to interaction: {}", why);
                        };
                    }.await;
                "".to_string()
                },
            };
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let _ = match msg.content.as_str() {
            "~ping" => commands::ping::message(ctx, msg).await,
            _ => ()
        };
    }


    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

       let _commands =  Command::create_global_application_command(&ctx.http, |command| commands::ping::register(command)).await;

       println!("Registered command!");
    }

}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
