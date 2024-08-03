use tracing::info;

use crate::{Context, Error};

/// Resets the bot's conversation memory
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn forget(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data().cleverbot.clear_context().await;

    ctx.say("Memory wiped").await?;

    Ok(())
}

/// Look into the bot's conversation memory
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn cerebroscopy(ctx: Context<'_>) -> Result<(), Error> {
    let memory = format!("Memory: {:?}", ctx.data().cleverbot.get_context().await.list)
        .chars()
        .take(1800)
        .chain("...".chars())
        .collect::<String>();

    info!("memory: {}\nlength: {}", memory, memory.len());

    ctx.say(format!("```{}```", memory)).await?;

    Ok(())
}

/// Re-generate the bot's cleverbot cookie
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn recookie(ctx: Context<'_>) -> Result<(), Error> {
    let res = ctx.data().cleverbot.generate_cookie().await?;
    info!("New cookie assigned successfully: {}", res);

    ctx.say("Thanks for the cookie sir!").await?;

    Ok(())
}