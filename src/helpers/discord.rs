use std::collections::HashMap;
use poise::{serenity_prelude::{ChannelId, ChannelType, Color, ComponentInteractionCollector, ComponentInteractionDataKind, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditMessage, GuildChannel}, CreateReply};
use regex::Regex;
use crate::{Context, Error};



pub async fn send_embed_menu(
    ctx: Context<'_>, 
    sections: Vec<(CreateSelectMenuOption, CreateEmbed)>, 
    initial_embed: CreateEmbed,
    menu_placeholder: &str
) -> Result<(), Error> {
    let (options, embeds): (Vec<_>, Vec<_>) = sections.into_iter().unzip();

    // setting the values to indeces (will be used later to associate menu options with embeds)
    let options = options.into_iter()
        .enumerate()
        .map(|(index, option)| {
            option.value(index.to_string())
        })
        .collect::<Vec<_>>();

    let ctx_id = ctx.id();
    let menu_id = format!("{}menu", ctx_id);

    let menu = CreateSelectMenu::new(
        &menu_id, 
        CreateSelectMenuKind::String { options }
    )
    .placeholder(menu_placeholder);

    let mut message = ctx.send(
        CreateReply::default()
            .components(vec![
                CreateActionRow::SelectMenu(menu.clone())
            ])
            .embed(initial_embed)
    )
    .await?
    .into_message()
    .await?;

    while let Some(interaction) = ComponentInteractionCollector::new(ctx)
        .filter(move |interaction| interaction.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60))
        .await
    {
        if let ComponentInteractionDataKind::StringSelect { ref values } = interaction.data.kind {
            let choice_index = values[0].parse::<usize>().unwrap();

            interaction.create_response(
                ctx.serenity_context(), 
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(embeds[choice_index].clone())
                )
            )
            .await?;
        }
    }

    // disabling the select menu after the timeout
    message.edit(
        ctx.serenity_context(),
        EditMessage::new()
            .components(vec![
                CreateActionRow::SelectMenu(menu.disabled(true))
            ])
    )
    .await?;

    Ok(())
}


pub fn filter_channels_by_type(channels: &HashMap<ChannelId, GuildChannel>, channel_type: ChannelType) -> Vec<&GuildChannel> {
    channels
        .iter()
        .filter_map(|(_, channel)| {
            if channel.kind == channel_type {
                return Some(channel);
            }
            None
        })
        .collect()
}

fn is_valid_hex_color(input: &str) -> bool {
    let hex_color_regex = Regex::new(r#"((0x)|(#))?[\dA-Fa-f]{6}"#).unwrap();
    hex_color_regex.is_match(input)
}

pub fn color_from_hex_str(input: &str) -> Result<Color, Error> {
    if !is_valid_hex_color(input) {
        Err("The hex value found didn't have enough digits (yes 6 is a requirement, # and 0x prefixes are allowed though)")?;
    }

    let trimmed_input = input.trim_start_matches(|c| c == '#' || c == '0').trim_start_matches("x");

    let color = u32::from_str_radix(trimmed_input, 16)
        .map(|hex_value| Color::from(hex_value))?;

    Ok(color)
}