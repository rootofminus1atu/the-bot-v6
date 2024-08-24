use std::{collections::HashMap, sync::Arc};
use anyhow::Context as _;
use commands::fun::minesweeper::Coord;
use poise::serenity_prelude::{self as serenity, ClientBuilder};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::{mpsc, Mutex};
use tracing::info;
use dashmap::DashMap;

pub mod error;
mod helpers;
mod events;
mod commands;
mod db_access;
mod model;


use helpers::cleverbot::Cleverbot;

struct CartChannel {
    sender: mpsc::Sender<String>,
}

pub struct Data {
    db: PgPool,
    client: reqwest::Client,
    translation_key: String,
    cleverbot: Arc<Cleverbot>,

    carts: DashMap<(serenity::UserId, serenity::ChannelId), mpsc::Sender<String>>,
    minesweepers: DashMap<(serenity::UserId, serenity::ChannelId), mpsc::Sender<Coord>>

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

    let cleverbot = Arc::new(helpers::cleverbot::Cleverbot::new(
        "lol_key_legit_there_is_no_key_requirement_yet".into(),
        100
    ));
    let cookie = cleverbot.generate_cookie().await
        .expect("Could not generate the 1st initial cookie");

    let client = reqwest::Client::new();


    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
                commands: vec![

                commands::fun::bite::bite(),
                commands::fun::calculate::calculate(),
                commands::fun::kazakhstan::kazakhstan(),
                commands::fun::translate::translate(),
                commands::fun::soy::soy(),
                // wip
                commands::fun::minesweeper::boysweeper(),
                // testing
                commands::fun::minesweeper::shop(),
                commands::fun::minesweeper::add(),
                
                commands::randomizer::animal::fox(),
                commands::randomizer::popequote::popequote(),
                commands::randomizer::palette::palette(),
        
                commands::info::help::help(),
        
                commands::admin::timed_message_config::admin(),
        
                commands::owner::cleverbot::forget(),
                commands::owner::cleverbot::cerebroscopy(),
                commands::owner::cleverbot::recookie(),
                commands::owner::kill::kill(),
            ],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(events::handler::event_handler(_ctx, event, _framework, _data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                mention_as_prefix: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let coms = &framework.options().commands;
                println!("Syncing {} commands", coms.len());
                poise::builtins::register_globally(ctx, coms).await?;

                println!("setting up pope messages");
                events::pope_msg::schedule_pope_msg(ctx.clone(), db.clone(), client.clone());

                println!("starting activity cycle");
                tokio::spawn(events::activity::change_activity(ctx.clone()));

                println!("starting clairvoyance");
                tokio::spawn(events::clairvoyance::clairvoyance(ctx.clone(), db.clone()));

                Ok(Data {
                    db,
                    client,
                    translation_key,
                    cleverbot,
                    carts: DashMap::new(),
                    minesweepers: DashMap::new()
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
