use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::channel::Message;
use serenity::Result as SerenityResult;

use crate::utils::interaction_message_response::interaction_message_response;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context, _options:&[CommandDataOption]) -> String {
    let temp_guild_id = command.guild_id.expect("Command not used in a guild!");
    let guild = ctx.cache.guild(temp_guild_id.as_u64().to_owned()).expect("Guild not found!");
    let guild_id = guild.id;

    let channel_id = guild.voice_states.get(&command.user.id)
            .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            interaction_message_response(command, ctx, "You are not in a voice channel!").await;
            return "Failed to connect!".to_string();
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialization").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    interaction_message_response(command, ctx, "Joined channel!").await;
    "Not implemented!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("joins the call!")
}

pub async fn message(ctx: Context, msg: Message) {
    if msg.content == "~join" {
        let guild = msg.guild(&ctx).expect("Failed to get guild!");
        let guild_id = guild.id;

        let channel_id = guild
            .voice_states.get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                check_msg(msg.reply(&ctx, "Not in a voice channel").await);

                return ();
            },
        };

        let manager = songbird::get(&ctx).await.expect("Songbird Voice client placed in at initialization").clone();

        let _handler = manager.join(guild_id, connect_to).await;

        ()
    }
}

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
