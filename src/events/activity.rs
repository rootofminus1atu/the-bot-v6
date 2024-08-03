use std::time::Duration;
use poise::serenity_prelude::{self as serenity, ActivityData};
use tokio;


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



