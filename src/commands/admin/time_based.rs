
use poise::serenity_prelude::ChannelId;
use crate::{model::pope_msg_location::{Location, PopeMsgCtx}, Context, Error};


// timed msgs
// pope
// clairvoyance
// cmd structure:
// config popemsg channel/language
// config clairvoyance channel


// TODO:
// check if owners_only and admins_only work in an AND or OR way (probably an AND way)

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("pope_msg"),
    subcommand_required,
    required_permissions = "MANAGE_GUILD",
    owners_only
)]
pub async fn config(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("channel"),
    subcommand_required,
    required_permissions = "MANAGE_GUILD",
    owners_only
)]
pub async fn pope_msg(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Toggles the channel to send daily pope messages to
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    owners_only
)]
pub async fn channel(ctx: Context<'_>, channel_id: ChannelId) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().map(|i| i.into()).ok_or("You can only use this in guilds")?;
    let channel_id: i64 = channel_id.into();
    let db = &ctx.data().db;

    let insert_res = Location::insert::<PopeMsgCtx>(db, guild_id, channel_id).await;

    // let delete_res = if let Err(delete_err) = insert_res {
    //     PopeMsgLocation::delete(db, guild_id, channel_id).await?.ok_or("Some db error")
    // };

    let delete_res = if insert_res.is_ok() { Err ("") } else {
        let res = Location::delete::<PopeMsgCtx>(db, guild_id, channel_id).await;

        if let Ok(Some(deleted)) = res {
            Ok(deleted)
        } else {
            Err ("")
        }
    };

    let msg = match (insert_res, delete_res) {
        (Ok(_), Err(_)) => "Channel set",
        (Err(_), Ok(_)) => "Channel unset",
        _ => "Some database error occurred."
    };

    ctx.say(msg).await?;

    Ok(())
}
