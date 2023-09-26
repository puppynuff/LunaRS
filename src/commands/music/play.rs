use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::application_command::CommandDataOption;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::channel::Message;

use crate::utils::{interaction_message_response::interaction_message_response, check_msg::{self, check_msg}};
pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context, options: &[CommandDataOption]) -> String {
    let string = options.get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    if let CommandDataOptionValue::String(url) = string {
        if !url.starts_with("http") {
            interaction_message_response(command, ctx, "URL argument needs to be http:// or https://").await;

            return "".to_string();
        }

        let temp_guild_id = command.guild_id.expect("Command not used in a guild!");
        let guild = ctx.cache.guild(temp_guild_id.as_u64().to_owned()).expect("Guild not found!");
        let guild_id = guild.id;

        let manager = songbird::get(ctx).await
            .expect("Songbird Voice client placed in a initialization.").clone();

        if let Some(handler_lock) = manager.get(guild_id) {
            let mut handler = handler_lock.lock().await;

            let source = match songbird::ytdl(&url).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {:?}", why);

                    interaction_message_response(command, ctx, "Error sourcing FFMPEG").await;

                    return "".to_string();
                }
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(format!("Playing {:?}~", source.metadata.title.clone().expect("Error getting title!")))
                                .embed(|embed| {
                                    embed.title("Queued Song!")
                                        .description(format!("Position : {:?}",  handler.queue().len()))
                                        .thumbnail(source.metadata.thumbnail.clone().expect("Failed to get thumbnail"))
                                        .fields(vec![
                                            ("Title: ", format!("{:?}", source.metadata.title.clone().expect("Failed to get title!")), false),
                                            ("Duration: ", format!("{:?}", source.metadata.duration.clone().expect("Failed to get duration")), false),
                                            ("Artist:", format!("{:?}", source.metadata.channel.clone().expect("Failed to get author!")), false)
                                    ])
                                })
                        })
                }).await {
                    interaction_message_response(command, ctx, "Failed to add song!").await;
                    println!("Error sending queue message: {}", why);
                }

            handler.enqueue_source(source.into());


        } else {
            interaction_message_response(command, ctx, "Not in a voice channel to play songs in").await;
        }
    } else {
        interaction_message_response(command, ctx, "Please input a valid string~").await;
    }

    "".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Plays music from youtube").create_option(|option| {
        option.name("url")
        .description("Youtube link to play music from")
        .kind(CommandOptionType::String)
        .required(true)
    })
}

pub async fn message(ctx: Context, msg: Message, args: Vec<&str>) {
    if args.len() < 2 {
        check_msg::check_msg(msg.reply(&ctx, "Needs a second argument!").await);
        return ();
    }

    let url = args[1];

    if !url.starts_with("http") {
        check_msg(msg.reply(&ctx, "URL needs to be http:// or https://").await);

        return ();
    }

    let guild = msg.guild(&ctx.cache).clone().expect("Guild not found!");
    let guild_id = guild.id;

    let manager = songbird::get(&ctx).await
        .expect("Songbird Voice client placed in at initialization.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.reply(&ctx, "Error sourcing FFMPEG").await);

                return ();
            }
        };

        check_msg(msg.channel_id.send_message(&ctx, |message| {
            message.content(format!("Added {:?} to queue~", source.metadata.title.clone().expect("Error getting title!")))
                .embed(|embed| {
                    embed.title("Queued song!")
                        .description(format!("Position: {:?}", handler.queue().len()))
                        .thumbnail(source.metadata.thumbnail.clone().expect("failed to get thumbnail!"))
                        .fields(vec![
                            ("Title: ", format!("{:?}", source.metadata.title.clone().expect("Failed to get title!")), false),
                            ("Duration: ", format!("{:?}", source.metadata.duration.clone().expect("Failed to get song duration!")), false),
                            ("Artist: ", format!("{:?}", source.metadata.channel.clone().expect("Failed to get author!")), false)
                        ])
                })
        }).await);

        handler.enqueue_source(source.into());
    } else {
        check_msg(msg.reply(&ctx, "Not in a voice channel to play songs in~").await);
    }

    ()
}
