use crate::{Context, Error};
use shunting::{ShuntingParser, MathContext};

/// Calculate some math
/// 
/// Calculate a mathematical expression. Example expressions:
/// - 1 + 2 * 3
/// - (sin(9))^2 + (cos(9))^2
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