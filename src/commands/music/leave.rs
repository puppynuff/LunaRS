use serenity::builder::CreateApplicationCommand;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::prelude::Context;
use serenity::model::channel::Message;

use crate::utils::interaction_message_response::interaction_message_response;
use crate::utils::check_msg::check_msg;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context, options: &[CommandDataOptionValue]) {

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("leaves the call")
}

pub async fn message(ctx: Context, msg: Message) {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(&ctx).await
        .expect("Songbird Voice client placed in at initialization").clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel!").await);
    } else {
        check_msg(msg.reply(&ctx.http, "Not in a voice channel!").await);
    }

    ()
}
