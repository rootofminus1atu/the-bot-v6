use crate::{Context, Error};

const SASHLEY_LINK: &str = "https://cdn.discordapp.com/attachments/1010464562434285640/1012690887429591120/HIP___ANIMATION_MEME.mp4";


/// Fully useless command! Sends an animation meme!
#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn sashley(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(SASHLEY_LINK).await?;
    Ok(())
}