use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
