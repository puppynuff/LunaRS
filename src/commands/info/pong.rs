use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::prelude::Context;
use serenity::model::channel::Message;

use crate::utils::interaction_message_response::interaction_message_response;

pub async fn run (command: &ApplicationCommandInteraction, ctx: &Context, _options: &[CommandDataOption]) -> String {
    interaction_message_response(command, ctx, "Ping~").await;

    "Outputted!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("pong").description("A pong command")
}

pub async fn message(ctx: Context, msg: Message) {
    if msg.content == "~pong" {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Ping~").await {
            println!("Error sending message: {:?}", why);
        }
    }
}
