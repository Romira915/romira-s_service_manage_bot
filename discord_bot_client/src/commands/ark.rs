pub(crate) mod ark_first;
pub(crate) mod ark_fourth;
pub(crate) mod ark_second;
pub(crate) mod ark_third;

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
use ark_fourth::*;
use ark_second::*;
use ark_third::*;

const REQUEST_TIMEOUT: u64 = 30;

#[group]
#[sub_groups(arkfirst, arksecond, arkthird)]
#[commands(start, status, stop, restart)]
#[prefixes("ark")]
#[description = "ARK管理コマンド"]
pub struct Ark;

#[command]
#[description = "ARKサーバを起動する．args: int int int ... int"]
async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    for arg in args.iter::<u32>() {
        match arg {
            Ok(1) => ArkFirst::ark_command_exec(SystemctlCommand::Start, ctx, msg).await?,
            Ok(2) => ArkSecond::ark_command_exec(SystemctlCommand::Start, ctx, msg).await?,
            Ok(3) => ArkThird::ark_command_exec(SystemctlCommand::Start, ctx, msg).await?,
            Ok(4) => ArkFourth::ark_command_exec(SystemctlCommand::Start, ctx, msg).await?,
            _ => (),
        }
    }

    Ok(())
}

#[command]
#[description = "ARKサーバの状態を表示する．args: int int int ... int"]
async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    for arg in args.iter::<u32>() {
        match arg {
            Ok(1) => ArkFirst::ark_command_exec(SystemctlCommand::Status, ctx, msg).await?,
            Ok(2) => ArkSecond::ark_command_exec(SystemctlCommand::Status, ctx, msg).await?,
            Ok(3) => ArkThird::ark_command_exec(SystemctlCommand::Status, ctx, msg).await?,
            Ok(4) => ArkFourth::ark_command_exec(SystemctlCommand::Status, ctx, msg).await?,
            _ => (),
        }
    }

    Ok(())
}

#[command]
#[description = "ARKサーバを停止する．args: int int int ... int"]
async fn stop(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        ArkFirst::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?;
        ArkSecond::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?;
        ArkThird::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?;
        ArkFourth::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?;

        return Ok(());
    }

    for arg in args.iter::<u32>() {
        match arg {
            Ok(1) => ArkFirst::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?,
            Ok(2) => ArkSecond::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?,
            Ok(3) => ArkThird::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?,
            Ok(4) => ArkFourth::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await?,
            _ => (),
        }
    }

    Ok(())
}

#[command]
#[description = "ARKサーバを再起動する．args: int int int ... int"]
async fn restart(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    for arg in args.iter::<u32>() {
        match arg {
            Ok(1) => ArkFirst::ark_command_exec(SystemctlCommand::Restart, ctx, msg).await?,
            Ok(2) => ArkSecond::ark_command_exec(SystemctlCommand::Restart, ctx, msg).await?,
            Ok(3) => ArkThird::ark_command_exec(SystemctlCommand::Restart, ctx, msg).await?,
            Ok(4) => ArkFourth::ark_command_exec(SystemctlCommand::Restart, ctx, msg).await?,
            _ => (),
        }
    }

    Ok(())
}
