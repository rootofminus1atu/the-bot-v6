use poise::serenity_prelude::User;
use poise::serenity_prelude::Mentionable;

use crate::{Context, Error, helpers::misc::random_choice};

const BITE_GIFS: &[&str] = &[
    "https://tenor.com/view/cat-bite-funny-chomp-gif-16986241",
    "https://tenor.com/view/mikisi-kisi-kiss-gif-27218966",
    "https://tenor.com/view/funny-cat-bit-video-gif-14264780414888402835"
];

const DEFAULT_GIF: &str = "https://tenor.com/view/cat-bite-funny-chomp-gif-16986241";

/// Bite someone
#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn bite(
    ctx: Context<'_>,
    #[description = "Someone to bite"]
    user: User
) -> Result<(), Error> {
    let gif = random_choice(BITE_GIFS).copied().unwrap_or(DEFAULT_GIF);
    
    ctx.say(format!("bites {}", user.mention())).await?;
    ctx.say(gif).await?;

    Ok(())
}