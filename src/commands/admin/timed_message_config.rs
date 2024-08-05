
use tracing::error;
use crate::error::CustomError;
use crate::{Context, Error};
use crate::model::location::{Location, LocationKind, ToggleAction};
use crate::helpers::discord::admin_or_owner;

// /admin/pope_msg or clairvoyance/channel

#[poise::command(
    prefix_command,
    slash_command,
    ephemeral,
    category = "Admin",
    subcommands("pope_msg", "prophecy"),
    subcommand_required,
    check = "admin_or_owner"
)]
pub async fn admin(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    ephemeral,
    category = "Admin",
    subcommands("pope_msg_channel"),
    subcommand_required,
)]
pub async fn pope_msg(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// (ADMIN ONLY) Toggles the channel for daily pope messages.
/// 
/// Run this command in a channel to toggle the channel for receiving daily pope messages. 
/// Pope messages are sent each day at 21:37 Dublin time, they include a random pope quote.
#[poise::command(
    prefix_command,
    slash_command,
    ephemeral,
    category = "Admin",
    rename = "channel",
)]
pub async fn pope_msg_channel(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().map(|i| i.into()).ok_or(CustomError::GuildOnly)?;
    let channel_id: i64 = ctx.channel_id().into();
    let db = &ctx.data().db;

    let location = Location {
        guild_id, 
        channel_id,
        kind: LocationKind::PopeMsg
    };

    let res = Location::toggle(db, &location).await
        .map_err(|why| {
            error!("/admin/pope_msg/channel error: {}", why);
            CustomError::SomeDbError
        })?;

    let msg = match res {
        (ToggleAction::Inserted, _) => "Channel set. Expect the next pope message to be sent here at 21:37.",
        (ToggleAction::Deleted, _) => "Channel unset. No longer will you see daily pope messages here.",
    };

    ctx.say(msg).await?;

    Ok(())
}



#[poise::command(
    prefix_command,
    slash_command,
    ephemeral,
    category = "Admin",
    subcommands("prophecy_channel"),
    subcommand_required,
)]
pub async fn prophecy(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// (ADMIN ONLY) Toggles the channel for daily random prophecies
///
/// Run this command in a channel to toggle the channel for receiving clairvoyant prophecies. 
/// Prophecies are sent once every 16 - 32 hours, the time interval is random and varies.
#[poise::command(
    prefix_command,
    slash_command,
    ephemeral,
    category = "Admin",
    rename = "channel",
)]
pub async fn prophecy_channel(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id: i64 = ctx.guild_id().map(|i| i.into()).ok_or(CustomError::GuildOnly)?;
    let channel_id: i64 = ctx.channel_id().into();
    let db = &ctx.data().db;

    let location = Location {
        guild_id, 
        channel_id,
        kind: LocationKind::Clairvoyance
    };

    let res = Location::toggle(db, &location).await
        .map_err(|why| {
            error!("/admin/clairvoyance/channel error: {}", why);
            CustomError::SomeDbError
        })?;

    let msg = match res {
        (ToggleAction::Inserted, _) => "Channel set. Expect the next prophecy to be sent here.",
        (ToggleAction::Deleted, _) => "Channel unset. No longer will I yap in this channel.",
    };

    ctx.say(msg).await?;

    Ok(())
}

