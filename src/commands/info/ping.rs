use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::prelude::Context;
use serenity::model::channel::Message;

use crate::utils::interaction_message_response::interaction_message_response;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context, _options: &[CommandDataOption]) -> String {
    interaction_message_response(command, ctx, "Pong~").await;

    "outputted!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}

pub async fn message(ctx: Context, msg: Message, _args: Vec<&str>) {
    if msg.content == "~ping" {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error sending message: {:?}", why);
        }
    }
}
