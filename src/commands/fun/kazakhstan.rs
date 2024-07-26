use crate::{Context, Error};

const KAZAKH_LINK: &str = "https://cdn.discordapp.com/attachments/1020620787289423892/1058706507073589268/kazakh.mp4"; 


/// Kazakhstan Grozi nam Bombardowaniem!
#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn kazakhstan(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(KAZAKH_LINK).await?;
    Ok(())
}