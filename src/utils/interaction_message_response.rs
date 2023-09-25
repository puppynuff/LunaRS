use serenity::{model::application::interaction::application_command::ApplicationCommandInteraction, prelude::Context};


pub async fn interaction_message_response(command: &ApplicationCommandInteraction, ctx: &Context, msg: &str) {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(msg))
        }).await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}
