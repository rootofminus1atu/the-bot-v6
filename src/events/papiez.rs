use std::sync::Arc;

use chrono_tz::Europe::Dublin;
use poise::serenity_prelude::{self as serenity, client, ChannelId, Mentionable, UserId};
use sqlx::{Pool, Postgres};
use tokio_cron::{Scheduler, Job};
use crate::model::{pope_msg_location::PopeMsgLocation, popequote::PopeQuote};
use futures::future::join_all;

const PAPIEZ_EMOJI: &str = "<a:papaspin:1263955406917734431>";
const BACKUP_PAPIEZ_MESSAGE: &str = "<a:papaspin:1263955406917734431> 2137 <a:papaspin:1263955406917734431>";

// in db
// - server_id
// - channel_id
// composite key to allow many channels in the same server




pub fn schedule_papiez_msg(ctx: serenity::Context, db: Pool<Postgres>, client: reqwest::Client) {
    // the scheduled papiez msg for 21:37
    let mut scheduler = Scheduler::new_in_timezone(Dublin);


    scheduler.add(Job::named("papiez", "5 37 21 * * *", move || {
        send_papiez_msg(ctx.clone(), db.clone(), client.clone())  // fucking double clone
    }));
}

async fn send_papiez_msg(ctx: serenity::Context, db: Pool<Postgres>, client: reqwest::Client) {
    let papiez_message = match PopeQuote::get_random(&client).await {
        Ok(pq) => format!("{} {} {}", PAPIEZ_EMOJI, pq.translation ,PAPIEZ_EMOJI),
        Err(why) => {
            eprintln!("Failed to get a random popequote: {:?}", why);
            BACKUP_PAPIEZ_MESSAGE.into()
        }
    };

    let locations = match PopeMsgLocation::get_all(&db).await {
        Ok(locations) => locations,
        Err(why) => {
            eprintln!("Failed to fetch the list of guild/channel locations for the timed pope msg: {:?}", why);
            return;
        }
    };

    let http = ctx.http;

    let futures = locations.into_iter().map(|location| {
        let http = http.clone();
        let papiez_message = papiez_message.clone();
        async move {
            if let Err(why) = ChannelId::new(location.channel_id as u64).say(&http, &papiez_message).await {
                eprintln!("Failed to send message to channel {}: {:?}", location.channel_id, why);
            }
        }
    });

    join_all(futures).await;
}