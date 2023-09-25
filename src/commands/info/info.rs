use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::model::Timestamp;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context, _options: &[CommandDataOption]) -> String {
    if let Err(why ) = command
        .create_interaction_response(&ctx.http, |response| {
            response.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("Luna_ info")
                    .embed(|embed| {
                        embed.title("Luna_ Info")
                            .description("Developer: PuppyNuff")
                            .image("https://cdn.discordapp.com/avatars/1129166787028734092/a76838e2f9d27080283577d7255b89d.webp") // I manually typed this out
                            .fields(vec![
                            ("Developer:", "PuppyNuff (Shiro)", true),
                            ("prefix", "~", true),
                            ("github", "https://github.com/puppynuff/LunaRS", false),
                            ("Discord", "Sorry, Currently there is no support server!", false),
                            ])
                            .footer(|footer| footer.text("Made using Serenity & rust!"))
                            .timestamp(Timestamp::now())
                    })
                })
        }).await
    {
        println!("Failed to run command: {:?}", why);
    };

    "Not made yet!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("info").description("Sends info on the discord bot")
}

pub async fn message(ctx: Context, msg: Message) {
    if msg.content == "~info" {

        if let Err(why) = msg.channel_id.send_message(&ctx.http, |message| {
            message.content("Luna_ info")
                .embed(|embed| {
                    embed.title("Luna_ Info")
                        .description("Developer: PuppyNuff")
                        .image("https://cdn.discordapp.com/avatars/1129166787028734092/a76838e2f9d27080283577d7255b89d.webp") // I manually typed this out
                        .fields(vec![
                        ("Developer:", "PuppyNuff (Shiro)", true),
                        ("prefix", "~", true),
                        ("github", "https://github.com/puppynuff/LunaRS", false),
                        ("Discord", "Sorry, Currently there is no support server!", false),
                        ])
                        .footer(|footer| footer.text("Made using Serenity & rust!"))
                        .timestamp(Timestamp::now())
                })
        }).await {
            println!("Error sending message: {:?}", why);
        };
    }
}
