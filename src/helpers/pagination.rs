use poise::{serenity_prelude::{Color, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage}, CreateReply};
use crate::{Context, Error, db_access::model_trait::Model};



const DB_ACCESS_EMBED_CHAR_LIMIT: i32 = 4000;

pub async fn db_access_embed_all<T: Model>(ctx: Context<'_>, items: Vec<T>) -> Result<(), Error> {

    let strs = items.iter()
        .map(|s| s.stringify())
        .collect::<Vec<_>>();

    let groups = divide_with_strlen(strs, DB_ACCESS_EMBED_CHAR_LIMIT);

    let pages = groups.iter()
        .map(|v| v.join("\n"))
        .collect::<Vec<_>>();

    let refed_pages = pages.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let embed_base = CreateEmbed::default()
        .title(format!("Displaying all {}", T::NAME_PLURAL))
        .color(Color::BLURPLE);

    paginate_str_pages(ctx, &refed_pages, &embed_base).await?;


    Ok(())
}

pub async fn paginate_str_pages(ctx: Context<'_>, pages: &[&str], embed_base: &CreateEmbed) -> Result<(), Error> {
    if pages.len() <= 0 {
        Err("Got an empty array, cannot paginate :(")?;
    }
    
    if pages.len() == 1 {
        ctx.send(
            CreateReply::default()
                .embed(
                    embed_base.clone()
                        .description(pages[0])
                )
        ).await?;

        return Ok(())
    }
    
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    let prev_button = CreateButton::new(&prev_button_id).emoji('◀');
    let next_button = CreateButton::new(&next_button_id).emoji('▶');

    let mut current_page = 0;

    let mut message = ctx.send(
        CreateReply::default()
            .embed(
                embed_base.clone()
                    .description(pages[current_page])
                    .footer(CreateEmbedFooter::new(format!("Page {}/{}", current_page + 1, pages.len())))
            )
            .components(vec![
                CreateActionRow::Buttons(vec![
                    prev_button.clone(),
                    next_button.clone()
                ])
            ])
    )
    .await?
    .into_message()
    .await?;


    while let Some(press) = ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60))
        .await 
    {
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            continue;
        }

        press.create_response(
            ctx.serenity_context(), 
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(
                        embed_base.clone()
                            .description(pages[current_page])
                            .footer(CreateEmbedFooter::new(format!("Page {}/{}", current_page + 1, pages.len())))
                    )
            )
        )
        .await?;
    }

    message.edit(
        ctx, 
        EditMessage::new()
        .components(vec![
            CreateActionRow::Buttons(vec![
                prev_button.disabled(true),
                next_button.disabled(true)
            ])
        ])
    )
    .await?;

    Ok(())
}


/// divides a list of strings into a list of lists of strings,
/// such that after concatenating said strings the total length for each collection
/// wouldn't excede the str_limit parameter
fn divide_with_strlen(list: Vec<String>, str_limit: i32) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current_page = Vec::new();
    let mut current_length = 0;

    for item in list {
        let item_length = item.len() as i32;
        if current_length + item_length > str_limit {
            result.push(std::mem::take(&mut current_page));
            current_length = 0;
        }

        current_page.push(item);
        current_length += item_length;
    }

    if !current_page.is_empty() {
        result.push(current_page);
    }

    result
}