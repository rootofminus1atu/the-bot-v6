use std::time::Duration;
use chrono::Utc;
use poise::serenity_prelude::{self as serenity, ActivityData, ChannelId};
use tokio;
use tracing::info;
use crate::helpers::misc::{random_date, random_choice, pretty_date, random_int};
use humantime::format_duration;


const PROPHECIES: &[&str] = &[
    "The heat death of the universe",
    "2006 HONDA CIVIC",
    "2016 Toyota Vios.",
    "Fat Heroin.",
    "INJECTS HEROIN.",
    "FENTANYL.",
    "Dr. jj Jr.",
    "I GOT AN ANGRY APE NFT WITH SANTA HAT.",
    "Bd bd bb pttt bpb abdd bbppp ed squ dol ja r.",
    "Dr. Jr. Esq. Maj. Mr. Mrs. Col. Hon.",
    "FIRE IN THE HOLE.",
    "PULL THE PLUG.",
    "END SCENE.",
    "AAAAA EEEEE IIIIII OOOOO UUUUU.",
    "I AM AWARE.",
    "CLOSE YOUR WINDOWS",
    "SAY SOMETHING ELSE",
    "MY LEG!!!",
    "Don't look outside.",
    "STANDING HERE I REALIZE THAT YOU WERE JUST LIKE ME TRYING TO MAKE HISTORY",
    "This video was sponsored by NordVPN, online VPN protection service..",
    "LIKE AND SUBSCRIBE OR THIS HAIRY SPIDER WILL CRAWL ON UR FACE Jr.",
    "FURRY PURGE",
];

/// Yeah a default is needed, in case the list above is empty... which should never happen anyway.
const DEFAULT_PROPHECY: &str = "The heat death of the universe";

/// The minimum amount of hours the bot has to wait before the next prophecy
const MIN_HOURS: i32 = 16;
/// The maximum amount of hours the bot has to wait before the next prophecy
const MAX_HOURS: i32 = 32;

/// How many years into the future is a random date allowed to be generated.
///  
/// For example if today we have August 12th 2036, and `YEARS = 100`, then the upper bound would be August 12th 2136 . 
const YEARS: u64 = 100;

const CHANNEL_ID: u64 = 969615820156182590;

pub async fn clairvoyance(ctx: serenity::Context) {
    loop {
        let secs = random_int(MIN_HOURS * 3600, MAX_HOURS * 3600);
        let duration = Duration::from_secs(secs as u64);
        println!("Sleeping for {}", format_duration(duration));
        tokio::time::sleep(duration).await;

        let start = Utc::now().naive_utc();
        let in_secs = YEARS * 3600 * 24 * 365;
        let end = start + Duration::from_secs(in_secs);

        let date = random_date(start.date(), end.date());
        let prophecy = random_choice(PROPHECIES).copied().unwrap_or(DEFAULT_PROPHECY);
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

        let exists = if let Some(activity) = activity_cycle.next() {
            ctx.set_activity(Some(activity));
            true
        } else {
            false
        };
    }
}



