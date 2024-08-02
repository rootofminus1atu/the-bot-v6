use poise::serenity_prelude::{Color, CreateEmbed, CreateEmbedAuthor};
use poise::CreateReply;
use crate::model::popequote::PopeQuote;
use crate::{Context, Error};


/// Pope John Paul the 2nd's wisdom
#[poise::command(slash_command, prefix_command, category = "Randomizer")]
pub async fn popequote(ctx: Context<'_>) -> Result<(), Error> {
    let q = PopeQuote::get_random(&ctx.data().client).await?;

    ctx.send(CreateReply::default()
        .embed(CreateEmbed::new()
            .author(CreateEmbedAuthor::new("John Paul the 2nd")
                .icon_url("https://media.discordapp.net/attachments/1060711805028155453/1060713256576106606/sc2.png?width=390&height=390")
            )
            .title("Quote:")
            .description(format!("*{}*", q.quote))
            .field(
                "Quote translation:", 
                format!("*{}*", q.translation), 
                true)
            .color(Color::BLURPLE)
        )
    ).await?;

    Ok(())
}