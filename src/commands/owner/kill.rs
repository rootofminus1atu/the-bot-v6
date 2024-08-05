use crate::{Context, Error};


#[poise::command(prefix_command, owners_only, hide_in_help, category = "Owner")]
pub async fn kill(ctx: Context<'_>) -> Result<(), Error> {
    let _ = ctx.say("Roses are red, I'm going to bed").await;

    ctx.framework()
        .shard_manager()
        .shutdown_all()
        .await;
    
    Ok(())
}