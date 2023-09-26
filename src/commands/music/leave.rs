use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::prelude::Context;
use serenity::model::channel::Message;

use crate::utils::interaction_message_response::interaction_message_response;
use crate::utils::check_msg::check_msg;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context, _options: &[CommandDataOption]) -> String {
    let temp_guild_id = command.guild_id.expect("Command not used in a guild!");
    let guild = ctx.cache.guild(temp_guild_id.as_u64().to_owned()).expect("Guild not found!");
    let guild_id = guild.id;

    let manager = songbird::get(&ctx).await
        .expect("Songbird Voice Client placed in at initialization").clone();

    let has_handler = manager.get(guild_id.to_owned()).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id.to_owned()).await {
            interaction_message_response(command, ctx, format!("Failed: {:?}", e).as_str()).await;
            return "".to_string();
        }

        interaction_message_response(command, ctx, "Left voice channel!").await;
        return "".to_string();
    }

    interaction_message_response(command, ctx, "Not in a voice channel!").await;
    return "".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("leaves the call")
}

pub async fn message(ctx: Context, msg: Message, _args: Vec<&str>) {
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
