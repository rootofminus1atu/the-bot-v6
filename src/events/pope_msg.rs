use chrono_tz::Europe::Dublin;
use poise::serenity_prelude::{self as serenity, ChannelId};
use sqlx::{Pool, Postgres};
use tokio_cron::{Scheduler, Job};
use futures::future::join_all;
use tracing::error;

use crate::model::{location::{Location, LocationKind}, popequote::PopeQuote};

const POPE_EMOJI: &str = "<a:papaspin:1263955406917734431>";
const BACKUP_POPE_MESSAGE: &str = "<a:papaspin:1263955406917734431> 2137 <a:papaspin:1263955406917734431>";



pub fn schedule_pope_msg(ctx: serenity::Context, db: Pool<Postgres>, client: reqwest::Client) {
    // the scheduled pope msg for 21:37
    let mut scheduler = Scheduler::new_in_timezone(Dublin);


    scheduler.add(Job::named("papiez", "5 37 21 * * *", move || {
        send_pope_msg(ctx.clone(), db.clone(), client.clone())  // fucking double clone
    }));
}

async fn send_pope_msg(ctx: serenity::Context, db: Pool<Postgres>, client: reqwest::Client) {
    let pope_message = match PopeQuote::get_random(&client).await {
        Ok(pq) => format!("{} {} {}", POPE_EMOJI, pq.translation ,POPE_EMOJI),
        Err(why) => {
            error!("Failed to get a random popequote: {:?}", why);
            BACKUP_POPE_MESSAGE.into()
        }
    };

    let locations = match Location::get_all(&db, LocationKind::PopeMsg).await {
        Ok(locations) => locations,
        Err(why) => {
            error!("Failed to fetch the list of guild/channel locations for the timed pope msg: {:?}", why);
            return;
        }
    };

    let http = ctx.http;

    let futures = locations.into_iter().map(|location| {
        let http = http.clone();
        let pope_message = pope_message.clone();
        async move {
            if let Err(why) = ChannelId::new(location.channel_id as u64).say(&http, &pope_message).await {
                error!("Failed to send pope message to channel {}: {:?}", location.channel_id, why);
            }
        }
    });

    join_all(futures).await;
}