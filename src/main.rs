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
            let _result = match command.data.name.as_str() {
                "ping" => commands::info::ping::run(&command, &ctx, &command.data.options).await,
                "info" => commands::info::info::run(&command, &ctx, &command.data.options).await,
                "pong" => commands::info::pong::run(&command, &ctx, &command.data.options).await,
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
            "~ping" => commands::info::ping::message(ctx, msg).await,
            "~info" => commands::info::info::message(ctx, msg).await,
            "~pong" => commands::info::pong::message(ctx, msg).await,
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
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
