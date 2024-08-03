use std::time::Duration;
use chrono::Utc;
use poise::serenity_prelude::{self as serenity, ActivityData, ChannelId};
use sqlx::PgPool;
use tokio;
use crate::{helpers::misc::{pretty_date, random_date, random_int}, model::prophecy::Prophecy};
use humantime::format_duration;


/// A default prophecy, in case we don't get any from the db.
const DEFAULT_PROPHECY: &str = "The heat death of the universe";

/// The minimum amount of hours the bot has to wait before the next prophecy
const MIN_HOURS: i32 = 16;
/// The maximum amount of hours the bot has to wait before the next prophecy
const MAX_HOURS: i32 = 32;

/// How many years into the future is a random date allowed to be generated.
///  
/// For example if today we have August 12th 2036, and `YEARS = 100`, then the upper bound would be August 12th 2136 . 
const YEARS: u64 = 100;

// const CHANNEL_ID: u64 = 969615820156182590;
const CHANNEL_ID: u64 = 1269047326358507591;

pub async fn clairvoyance(ctx: serenity::Context, db: PgPool) {
    loop {
        let secs = random_int(MIN_HOURS * 3600, MAX_HOURS * 3600);
        let duration = Duration::from_secs(secs as u64);
        println!("Sleeping for {}", format_duration(duration));
        tokio::time::sleep(duration).await;

        let start = Utc::now().naive_utc();
        let in_secs = YEARS * 3600 * 24 * 365;
        let end = start + Duration::from_secs(in_secs);

        let date = random_date(start.date(), end.date());
        // let prophecy = random_choice(PROPHECIES).copied().unwrap_or(DEFAULT_PROPHECY);
        let prophecy = Prophecy::get_random(&db).await.map(|p| p.content).unwrap_or(DEFAULT_PROPHECY.into());
        let msg = format!("{}, {}", pretty_date(&date), prophecy);

        
        // in the future get channels from the db instead of hardcoding them here
        let channel_id = ChannelId::from(CHANNEL_ID); 

        if let Err(why) = channel_id.say(&ctx, msg).await {
            eprintln!("Failed to send clairvoyance message: {:?}", why);
        }
    }
}



pub async fn change_activity(ctx: serenity::Context) {
    let activities = vec![
        ActivityData::watching(":3"),
        ActivityData::playing(":3"),
    ];

    let mut activity_cycle = activities.into_iter().cycle();
    let mut timer = tokio::time::interval(Duration::from_secs(60));

    loop {
        timer.tick().await;

        let _exists = if let Some(activity) = activity_cycle.next() {
            ctx.set_activity(Some(activity));
            true
        } else {
            false
        };
    }
}



