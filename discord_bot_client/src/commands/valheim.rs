use std::process::{ExitStatus, Output};

use duct::cmd;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, prelude::Activity},
    utils::Colour,
};

use crate::commands::EmbedMessageBuilder;

#[group]
#[commands(start, status, stop, restart)]
#[prefixes("valheim", "vh")]
#[description = "Valheim管理コマンド"]
pub struct Valheim;

#[command]
#[description = "Valheimサーバを起動する"]
async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

    let embed = match cmd!(
        "az",
        "vm",
        "start",
        "--ids",
        cmd!(
            "az",
            "vm",
            "list",
            "-g",
            "Valheim",
            "--query",
            "\"[].id\"",
            "-o",
            "tsv"
        )
        .stdout_capture()
        .read()
        .unwrap_or_default(),
    )
    .stdout_capture()
    .stderr_capture()
    .run()
    {
        Ok(output) => {
            if output.status.success() {
                ctx.set_activity(Activity::playing("Valheim")).await;
                EmbedMessageBuilder::default().success(true).build()
            } else {
                EmbedMessageBuilder::default()
                    .success(false)
                    .message(String::from_utf8(output.stderr).unwrap())
                    .build()
            }
        }
        Err(why) => EmbedMessageBuilder::default()
            .success(false)
            .message(why.to_string())
            .build(),
    };

    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(embed))
        .await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "Valheimサーバの状態を表示する"]
async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

    let embed = match cmd!(
        "az",
        "vm",
        "get-instance-view",
        "--name",
        "ValheimServer",
        "--resource-group",
        "Valheim",
        "--query",
        "\"instanceView.statuses[1].displayStatus\"",
        "-o",
        "tsv"
    )
    .stdout_capture()
    .stderr_capture()
    .run()
    {
        Ok(output) => {
            if output.status.success() {
                EmbedMessageBuilder::default()
                    .success(true)
                    .message(
                        if String::from_utf8(output.stdout).unwrap_or_default().trim()
                            == "VM running"
                        {
                            "Valheim server running".to_string()
                        } else {
                            "Valheim server stopping".to_string()
                        },
                    )
                    .build()
            } else {
                EmbedMessageBuilder::default()
                    .success(true)
                    .message(String::from_utf8(output.stderr).unwrap())
                    .build()
            }
        }
        Err(why) => EmbedMessageBuilder::default()
            .success(false)
            .message(why.to_string())
            .build(),
    };

    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(embed))
        .await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "Valheimサーバを停止する"]
async fn stop(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

    let embed = match cmd!(
        "az",
        "vm",
        "deallocate",
        "--name",
        "ValheimServer",
        "--resource-group",
        "Valheim",
    )
    .stdout_capture()
    .stderr_capture()
    .run()
    {
        Ok(output) => {
            if output.status.success() {
                ctx.reset_presence().await;
                EmbedMessageBuilder::default().success(true).build()
            } else {
                EmbedMessageBuilder::default()
                    .success(true)
                    .message(String::from_utf8(output.stderr).unwrap())
                    .build()
            }
        }
        Err(why) => EmbedMessageBuilder::default()
            .success(false)
            .message(why.to_string())
            .build(),
    };

    msg.channel_id
        .send_message(&ctx.http, |m| m.set_embed(embed))
        .await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "Valheimサーバを再起動する"]
async fn restart(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("このコマンドは未実装です")
                    .description("すまんな")
                    .colour(Colour::BLITZ_BLUE)
                    .image(
                        "https://matome.hacker-hacker.com/wp-content/uploads/2020/05/genba-1.gif",
                    )
            })
        })
        .await?;

    typing.stop();

    Ok(())
}
