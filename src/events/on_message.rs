use std::sync::Arc;
use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;
use tracing::info;
use crate::{helpers::cleverbot::Cleverbot, Error};

const DEFAULT_RESPONSE: &str = "Hi";

pub async fn on_message(new_message: &Message, ctx: &serenity::Context, cleverbot: Arc<Cleverbot>) -> Result<(), Error> {
    // if the bot sends a message, don't do anything with it
    if new_message.author.id == ctx.cache.current_user().id {
        return Ok(())
    }
    
    if new_message.mentions_me(ctx).await? {
        let me = ctx.cache.current_user().clone();
        let id_string = format!("<@{}>", me.id);

        let formated_new_message = new_message.content.replace(&id_string, &me.name);

        let response = cleverbot.get_response(&formated_new_message).await?;


        info!("clev:\n- `{}`\n=> `{}`\n- `{}`", new_message.content, formated_new_message, response);

        if response == "<html" || response == "Hello from Cleverbot\n" {
            info!(" ====== BAD CLEV RESPONSE ====== ");
            let _res = cleverbot.generate_cookie().await?;
            new_message.reply(ctx, DEFAULT_RESPONSE).await?;
            return Ok(());
        }

        new_message.reply(ctx, response).await?;
    }

    Ok(())
}