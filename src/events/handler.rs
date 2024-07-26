use poise::serenity_prelude::{self as serenity};
use crate::{Data, Error};
use super::on_message::on_message;


pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        },
        serenity::FullEvent::Message { new_message } => on_message(new_message, ctx, 
            data.cleverbot.clone()
        ).await?,
        _ => {}
    }
    Ok(())
}



