use core::fmt;
use std::time::Duration;

use homeserver_receive_process::CommandBuilder;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::bot_config::ConfigContainer;

use super::EmbedMessageBuilder;

const REQUEST_TIMEOUT: u64 = 30;

#[group]
#[commands(start, status, stop, restart)]
#[prefixes("minecraft", "mc")]
#[description = "Minecraft管理コマンド"]
pub struct Minecraft;

impl Minecraft {
    async fn generate_url(ctx: &Context) -> String {
        let data_read = ctx.data.read().await;
        let config = data_read
            .get::<ConfigContainer>()
            .expect("Expected ConfigContainer in TypeMap");

        let address = config.address();
        format!(
            "http://{}:{}/minecraft",
            address.home_server_ip(),
            address.home_server_port()
        )
    }

    async fn minecraft_command_exec(
        command: SystemctlCommand,
        ctx: &Context,
        msg: &Message,
    ) -> CommandResult {
        let typing = msg.channel_id.start_typing(&ctx.http).unwrap();
        let url = Minecraft::generate_url(ctx).await;
        let admin = match command {
            SystemctlCommand::Start | SystemctlCommand::Status => false,
            SystemctlCommand::Stop | SystemctlCommand::Restart => true,
        };

        let post_data = CommandBuilder::default()
            .request(command.to_string())
            .administrator(admin)
            .build()
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&post_data)
            .timeout(Duration::from_secs(REQUEST_TIMEOUT))
            .send()
            .await;

        let _ = match response {
            Ok(res) => {
                let success = res.status().is_success();
                let body = res.text().await.unwrap();

                match command {
                    SystemctlCommand::Start | SystemctlCommand::Restart => {
                        ctx.set_activity(Activity::playing("Minecraft")).await
                    }
                    SystemctlCommand::Stop => ctx.reset_presence().await,
                    _ => (),
                }

                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.set_embed(
                            EmbedMessageBuilder::default()
                                .success(success)
                                .message(body)
                                .build(),
                        )
                    })
                    .await?
            }
            Err(err) => {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.set_embed(
                            EmbedMessageBuilder::default()
                                .success(false)
                                .message(err.to_string())
                                .build(),
                        )
                    })
                    .await?
            }
        };

        typing.stop();

        Ok(())
    }
}

pub enum SystemctlCommand {
    Start,
    Status,
    Stop,
    Restart,
}

impl fmt::Display for SystemctlCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Start => write!(f, "start"),
            Self::Status => write!(f, "status"),
            Self::Stop => write!(f, "stop"),
            Self::Restart => write!(f, "restart"),
        }
    }
}

#[command]
#[description = "Minecraftサーバを起動する"]
async fn start(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    Minecraft::minecraft_command_exec(SystemctlCommand::Start, ctx, msg).await
}

#[command]
#[description = "Minecraftサーバの状態を表示する"]
async fn status(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    Minecraft::minecraft_command_exec(SystemctlCommand::Status, ctx, msg).await
}

#[command]
#[owners_only]
#[description = "Minecraftサーバを停止する"]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    Minecraft::minecraft_command_exec(SystemctlCommand::Stop, ctx, msg).await
}

#[command]
#[owners_only]
#[description = "Minecraftサーバを再起動する"]
async fn restart(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    Minecraft::minecraft_command_exec(SystemctlCommand::Restart, ctx, msg).await
}
