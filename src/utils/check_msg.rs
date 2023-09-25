use serenity::Result as SerenityResult;
use serenity::model::channel::Message;

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
