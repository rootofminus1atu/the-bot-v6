use crate::{Context, Error};
use shunting::{ShuntingParser, MathContext};

/// Cat knows some simple math too!
#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn calculate(
    ctx: Context<'_>,
    expression: String
) -> Result<(), Error> {

    let parsed = ShuntingParser::parse_str(&expression)?;
    let res = MathContext::new().eval(&parsed)?;

    ctx.say(format!("{} = {}", expression, res)).await?;

    Ok(())
}