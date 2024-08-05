use poise::{serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter, CreateSelectMenuOption, ReactionType}, Command};
use crate::{Context, Data, Error, helpers::discord::send_embed_menu};

/// See and get help on available commands
#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let categories = CmdCategoryCollection::from_default(
        // a default category is required, in case we forget to assign a command to a category or one doesn't fit into any
        CmdCategory::from("Misc", "❓", "All the other commands that don't belong to any category")
        )
        .add_categories(vec![
            CmdCategory::from("Fun", "🎉", "Commands made for fun"),
            CmdCategory::from("Randomizer", "🎲", "Random-oriented commands"),
            CmdCategory::from("Info", "ℹ️", "Information about the bot, server, and more"),
            CmdCategory::from("Admin", "🤓", "Admin commands, can be used by staff members only"),
            ])
        .fall_in(&ctx.framework().options().commands);

    let sections = categories.all()
        .iter()
        .enumerate()
        .map(|(i, category)| {

            let option = CreateSelectMenuOption::new(category.name, category.name)
                .description(category.description)
                .emoji(category.emoji.parse::<ReactionType>().unwrap());

            let embed = CreateEmbed::default()
                .title(format!("{}. {}", i + 1, &category.name))
                .description(category.description)
                .fields(category.commands.iter()
                    .filter(|com| 
                        !com.hide_in_help &&
                        (com.prefix_action.is_some() ||
                        com.slash_action.is_some()) &&
                        !com.subcommand_required
                    )
                    .map(|com| {
                        let com_pars_str = com.parameters.iter()
                            .map(|par| format!("[{}]", par.name))
                            .collect::<Vec<_>>()
                            .join(" ");

                        let title = format!("/{} {}", com.qualified_name, com_pars_str);
                        let desc = com.help_text.clone()
                            .or(com.description.clone())
                            .unwrap_or("".into());

                        (title, desc, false) 
                    })
                )
                .color(Color::BLURPLE)
                .footer(CreateEmbedFooter::new("You can select another help section from the dropdown menu below!"));

            (option, embed)
        })
        .collect::<Vec<_>>();
    

    let initial_embed = CreateEmbed::default()
        .title("Help menu")
        .description("Help desc")
        .color(Color::BLURPLE);

    send_embed_menu(ctx, &sections, initial_embed, "Select a help section!").await?;

    Ok(())
}


#[derive(Debug)]
pub struct CmdCategory<'a> {
    pub name: &'a str,
    pub emoji: &'a str,
    pub description: &'a str,
    pub commands: Vec<&'a Command<Data, Error>>,
}

impl<'a> CmdCategory<'a> {
    pub fn from(name: &'a str, emoji: &'a str, description: &'a str) -> Self {
        Self {
            name,
            emoji,
            description,
            commands: vec![],
        }
    }
}

#[derive(Debug)]
pub struct CmdCategoryCollection<'a> {
    pub categories: Vec<CmdCategory<'a>>,
    pub default: CmdCategory<'a>,
}

impl<'a> CmdCategoryCollection<'a> {
    pub fn from_default(default: CmdCategory<'a>) -> Self {
        Self {
            categories: vec![],
            default,
        }
    }

    pub fn add_categories(mut self, command_categories: Vec<CmdCategory<'a>>) -> Self {
        self.categories.extend(command_categories);
        self
    }

    pub fn fall_in(mut self, commands: &'a [Command<Data, Error>]) -> Self {
        fn traverse_and_process<'b>(categories: &mut Vec<CmdCategory<'b>>, default: &mut CmdCategory<'b>, cmd: &'b Command<Data, Error>) {
            let found = categories.iter_mut().find(|cat| {
                cmd.category.as_ref().is_some_and(|cat_name| cat_name == &cat.name)
            });
    
            match found {
                Some(cmd_category) => cmd_category.commands.push(cmd),
                None => default.commands.push(cmd),
            }
    
            for subcmd in &cmd.subcommands {
                traverse_and_process(categories, default, subcmd);
            }
        }
    
        for cmd in commands {
            traverse_and_process(&mut self.categories, &mut self.default, cmd);
        }
    
        self
    }

    pub fn all(&self) -> Vec<&CmdCategory<'a>> {
        let mut all_categories: Vec<&CmdCategory<'a>> = self.categories.iter().collect();
        all_categories.push(&self.default);
        all_categories
    }
}