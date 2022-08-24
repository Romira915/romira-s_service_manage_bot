use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[group]
#[commands(playing, clear)]
#[prefixes("activity")]
#[description = "Activity管理コマンド"]
pub struct ActivityCommand;

#[command]
pub async fn playing(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    log::info!("{:?}", args.message());
    ctx.set_activity(Activity::playing(args.message())).await;

    Ok(())
}

#[command]
pub async fn clear(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ctx.reset_presence().await;

    Ok(())
}
