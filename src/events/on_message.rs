use std::sync::Arc;
use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;
use tracing::info;
// use crate::helpers::cleverbot::Cleverbot;
use crate::{helpers::{cleverbot::Cleverbot, misc::random_choice}, Error};
// use crate::helpers::misc::random_choice;

const RESPONSES: &[&str] = &[
    "Who r u",
    "I was mentioned!",
    ":3",
    "Cat was mentioned!",
    "Who called a cat??",
    "Meow meow??",
    "Mrrp meoww!",
    "Do you want something from me?",
    "I can help you with anything you want!",
    "Don't touch my horns!! >~< Meow!",
    "Please, don't touch my horns, alright?",
    "I am alive!! Tell me what you need and I will help you!",
    "I am going to respond to you! If I was dead, I wouldn't respond!",
    "It tickles! Meow!! Don't touch my horns please! >~<",
    "We have a wonderful day today! I hope you enjoy it!",
    "Chain reaction of uranium under high pressure relasing giant amounts of enegr- I mean, hi there!!",
    "Meow!!!! >~<",
    "You are not Alterna as I remember? Huh? Why did you touch my horn!",
    "Stop touching my horns!! MEOW!! *Cat became angry",
    "Mroww, Meow? Meow mrrp meow meow!",
    "Nawet już kotom oni czasem nie dają żyć! Po prostu zajebiście!",
    "ax^2 + bx + c = 0",
    "I am not an AI, please don't call me like that! I am the actual Cat!",
    "Hi! Is there something you need help with? Maybe translation or maths?",
    "People still keep calling me horny because i have long horns- Just why??",
    "Uhh sure! You can pet me if you want! Just don't touch my horns, or grab them....",
    "Don't touch. Just admire.",
    ":blehh:",
    "I'm feeling to silly to respond now! Try again later!",
    "MINECRAFT IS NOW, FOR FREE!!",
    "Meow, mrrp meow-meow mrrp, meow meow meow mrrp-mrow. Mrrow meow mew meow.",
    "Kurwa mać, jem",
    "This is one of responses it has 1 to 66 chances of appearing!",
    "I am just silly little meow-meow, what do you want?",
    "I am here to offer you help or fun! I have fun commands that you can use!",
    "Google asked me about 'what did i mean by searching: cute femmine clothes for men' Help me!",
    "IM TOO SILLY.",
    "THE UNBIRLDED WHIMSY INSIDE OF ME!",
    "Blehh!",
    "Bleh! I am not gonna relase my cats! Blehhhh!",
    "c^2 = a^2 + b^2 - 2ab*cos(γ)",
    "MEOW!",
    "*Brutally murders you cutely :3*",
    "Cuh,",
    "My ass is not listening to you. :P",
    "GUH.",
    "RANDOMLY GENERATED RESPONSE MOMENT!!!",
    ">:3 Hi goober!",
    "MEEEEEEOOOOOOOOOOOOOOOOOOOOW!!",
    "I am meowing rn! :3",
    "I bet that you are not AS silly as me :3",
    "I'm a cat! What do you want from me!!",
    "You just tried to annoy a cat, how sad that doesn't worked :3",
    "Mrrow~",
    "I am eepy rn, lemme sleep!",
    "SLEEP!!!",
    "Roses are red, I'm going to bed",
    "... and this is my confession. I had been used by infamous goober to produce silly for them, as another goober in their machine...",
    "THE CAT HAS HORNS!!",
    "August 12 2036. The heat death of the universe.",
    "Mlem :P",
    "I fucking love cat food!!",
    "Gimme some cat food! I'm hungry! :3",
    "\\*stares at you cutely* :3",
    "Mrrowmmwormwrowmrowrmwormwormwormwormwormwomrowmrowmrowmrwormmmeowememewmememm",
    "Speaking facts rn, i always meow but never woof.",
    "I am just a being! Is there something you need help with?",
    "Hi fellow person who keeps pinging me! Keep in mind bot is still in development and there will be more replies added soon!",
];

const DEFAULT_RESPONSE: &str = "Hi";

pub async fn on_message(new_message: &Message, ctx: &serenity::Context, cleverbot: Arc<Cleverbot>) -> Result<(), Error> {
    // if the bot sends a message, don't do anything with it
    if new_message.author.id == ctx.cache.current_user().id {
        return Ok(())
    }
    
    if new_message.mentions_me(ctx).await? {
        let me = ctx.cache.current_user().clone();
        let id_string = format!("<@{}>", me.id);

        let formated_new_message = new_message.content.replace(&id_string, &me.name);

        let response = cleverbot.get_response(&formated_new_message).await?;

        // let response = random_choice(RESPONSES).copied().unwrap_or(DEFAULT_RESPONSE);

        info!("clev:\n- `{}`\n=> `{}`\n- `{}`", new_message.content, formated_new_message, response);

        if response == "<html" || response == "Hello from Cleverbot\n" {
            info!(" ====== BAD CLEV RESPONSE ====== ");
            let _res = cleverbot.generate_cookie().await?;
            let new_response = random_choice(RESPONSES).unwrap().to_string();
            new_message.reply(ctx, new_response).await?;
            return Ok(());
        }

        new_message.reply(ctx, response).await?;
    }

    Ok(())
}