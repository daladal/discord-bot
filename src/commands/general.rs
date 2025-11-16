use serenity::prelude::*;
use serenity::model::channel::Message;

pub async fn ping(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}

pub async fn help(ctx: &Context, msg: &Message) {
    let help_text = "Available commands:\n\
        ping - Responds with Pong!\n\
        help - Shows this message\n\
        prefix [new_prefix] - View or set command prefix for this server\n\
        link <Name#TAG> <region> - Link your Discord account to your LoL account\n\
        unlink - Remove your linked LoL account\n\
        me - Show your linked LoL account\n\
        \n\
        Regions: na, euw, eune, kr, br, lan, las, oce, ru, tr, jp, ph, sg, th, tw, vn\n\
        Example: !link Faker#KR1 kr\n\
        \n\
        Tip: Use quotes for multi-word arguments: `!command \"multi word arg\"`";
    
    if let Err(why) = msg.channel_id.say(&ctx.http, help_text).await {
        println!("Error sending message: {:?}", why);
    }
}
