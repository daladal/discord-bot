mod general;
mod config;
mod user;

use serenity::prelude::*;
use serenity::model::channel::Message;

pub async fn handle_command(ctx: &Context, msg: &Message, command: &str, args: Vec<String>) {
    match command {
        "ping" => general::ping(ctx, msg).await,
        "help" => general::help(ctx, msg).await,
        "prefix" => config::prefix(ctx, msg, args).await,
        "link" => user::link(ctx, msg, args).await,
        "unlink" => user::unlink(ctx, msg).await,
        "me" => user::me(ctx, msg).await,
        _ => {}
    }
}
