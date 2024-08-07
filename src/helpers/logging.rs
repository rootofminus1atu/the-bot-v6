use chrono::Utc;
use poise::serenity_prelude::{self as serenity, ChannelId, Color, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, Message, User};
use crate::helpers::misc::pretty_datetime;


const LOGGING_CHANNEL_ID: u64 = 1001558158768095252;

/// whenever the bot is mentioned this function will run
pub async fn mention_log(ctx: serenity::Context, new_message: Message) {

    let channel = ChannelId::from(LOGGING_CHANNEL_ID);

    let res = channel.send_message(ctx, CreateMessage::new()
        .embed(CreateEmbed::new()
            .author(CreateEmbedAuthor::new(&new_message.author.name)
                .icon_url(&new_message.author.face())
            )
            .description(format!("`{}` responded to me with: `{}`", new_message.author.name, new_message.content))
            .footer(CreateEmbedFooter::new(pretty_datetime(&Utc::now().naive_utc())))
            .color(Color::BLURPLE)
        )
    ).await;

    if let Err(why) = res {
        error!("I CAN'T LOG MENTION: {}", why);
    }
}


pub struct CmdLogInfo {
    pub ctx: serenity::Context,
    pub who: User,
    pub cmd_name: String,
    pub cmd_prefix: String,
    pub guild_name: String
}

/// Whenever a command is invoked (slash or prefix) this function will run
pub async fn command_log(info: CmdLogInfo) {
    // some db or other api stuff that can take time so i spawn a task

    let channel = ChannelId::from(LOGGING_CHANNEL_ID);

    let res = channel.send_message(info.ctx, CreateMessage::new()
        .embed(CreateEmbed::new()
            .author(CreateEmbedAuthor::new(&info.who.name)
                .icon_url(&info.who.face())
            )
            .description(format!("`{}` used the `{}{}` command in a server named `{}`", info.who.name, info.cmd_prefix, info.cmd_name, info.guild_name))
            .footer(CreateEmbedFooter::new(pretty_datetime(&Utc::now().naive_utc())))
            .color(Color::BLITZ_BLUE)
        )
    ).await;

    if let Err(why) = res {
        error!("I CAN'T LOG COMMAND: {}", why);
    }
}