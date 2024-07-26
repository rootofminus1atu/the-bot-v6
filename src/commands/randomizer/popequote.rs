use poise::serenity_prelude::{Color, CreateEmbed, CreateEmbedAuthor};
use poise::CreateReply;
use crate::{Context, Error};
use crate::db_access::popequote_model::PopeQuote;


/// Pope John Paul the 2nd's wisdom
#[poise::command(slash_command, prefix_command, category = "Randomizer")]
pub async fn popequote(ctx: Context<'_>) -> Result<(), Error> {
    let q = PopeQuote::get_random(&ctx.data().db).await?;

    ctx.send(CreateReply::default()
        .embed(CreateEmbed::new()
            .author(CreateEmbedAuthor::new("John Paul the 2nd")
                .icon_url("https://media.discordapp.net/attachments/1060711805028155453/1060713256576106606/sc2.png?width=390&height=390")
            )
            .title("Quote:")
            .description(format!("*{}*", q.pl))
            .field(
                "Quote translation:", 
                format!("*{}*", q.en), 
                true)
            .color(Color::BLURPLE)
        )
    ).await?;

    Ok(())
}