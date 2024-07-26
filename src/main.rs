use std::{collections::HashSet, sync::Arc};
use anyhow::Context as _;
use poise::serenity_prelude::{self as serenity, ClientBuilder, UserId};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;


mod helpers;
mod events;
mod commands;
mod db_access;


use helpers::cleverbot::Cleverbot;

pub struct Data {
    db: PgPool,
    translation_key: String,
    cleverbot: Arc<Cleverbot>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;




#[shuttle_runtime::main]
async fn poise(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let translation_key = secret_store
        .get("TRANSLATION_KEY")
        .context("No translation key found in environment variables")?;

    let database_url = secret_store
        .get("DATABASE_URL")
        .context("No database url found in environment variables")?;

    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");


    let cleverbot_api_link = secret_store
        .get("CLEVERBOT_API_LINK")
        .context("No cleverbot api link found in environment variables")?;

    let cleverbot = Arc::new(helpers::cleverbot::Cleverbot::new(
        "lol_key_legit".into(), 
        cleverbot_api_link, 
        100
    ));
    let cookie = cleverbot.generate_cookie().await
        .expect("Could not generate the 1st initial cookie");

    info!("Starting off with cookie: {}", cookie);


    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![

                commands::fun::bite::bite(),
                commands::fun::calculate::calculate(),
                commands::fun::kazakhstan::kazakhstan(),
                commands::fun::sashley::sashley(),
                
                commands::randomizer::animal::fox(),
                commands::randomizer::popequote::popequote()
                
                // owner
                // kill(),
                // forget(),
                // cerebroscopy(),
                // recookie(),

                // db_access
                // db_access::commands::owner()
            ],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(events::handler::event_handler(_ctx, event, _framework, _data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let coms = &framework.options().commands;
                println!("Syncing {} commands", coms.len());
                poise::builtins::register_globally(ctx, coms).await?;

                println!("setting up papiez messages");
                events::papiez::schedule_papiez_msg(ctx.clone(), db.clone());

                println!("starting activity cycle");
                tokio::spawn(events::time_based::change_activity(ctx.clone()));

                println!("starting clairvoyance");
                tokio::spawn(events::time_based::clairvoyance(ctx.clone()));

                Ok(Data {
                    db,
                    translation_key,
                    cleverbot
                })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, serenity::GatewayIntents::non_privileged() 
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
