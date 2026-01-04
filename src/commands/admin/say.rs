use crate::{Context, Error};
use poise::serenity_prelude::GuildChannel;

/// I shall say what you command me to say
#[poise::command(slash_command, ephemeral, owners_only, category = "Admin")]
pub async fn say(
    ctx: Context<'_>,
    #[description = "What I'll say"]
    text: String,
    #[description = "Where I'll say it"]
    #[channel_types("Text")]
    channel: GuildChannel
) -> Result<(), Error> {
    channel.say(ctx, text).await?;

    ctx.say("Message sent!").await?;

    Ok(())
}