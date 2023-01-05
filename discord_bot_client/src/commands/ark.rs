pub(crate) mod ark_first;
pub(crate) mod ark_second;

use std::time::Duration;

use homeserver_receive_process::CommandBuilder;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::bot_config::ConfigContainer;

use super::minecraft::SystemctlCommand;
use super::EmbedMessageBuilder;
use ark_first::*;
use ark_second::*;

const REQUEST_TIMEOUT: u64 = 30;

#[group]
#[sub_groups(arkfirst, arksecond)]
#[commands(start, status, stop, restart)]
#[prefixes("ark")]
#[description = "ARK管理コマンド"]
pub struct Ark;

#[command]
#[description = "ARKサーバを起動する"]
async fn start(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkFirst::ark_command_exec(SystemctlCommand::Start, ctx, msg).await?;
    ArkSecond::ark_command_exec(SystemctlCommand::Start, ctx, msg).await
}

#[command]
#[description = "ARKサーバの状態を表示する"]
async fn status(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkFirst::ark_command_exec(SystemctlCommand::Status, ctx, msg).await?;
    ArkSecond::ark_command_exec(SystemctlCommand::Status, ctx, msg).await
}

#[command]
#[description = "ARKサーバを停止する"]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkFirst::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?;
    ArkSecond::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await
}

#[command]
#[description = "ARKサーバを再起動する"]
async fn restart(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkFirst::ark_command_exec(SystemctlCommand::Restart, ctx, msg).await?;
    ArkSecond::ark_command_exec(SystemctlCommand::Restart, ctx, msg).await
}
