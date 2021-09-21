use core::fmt;
use std::borrow::BorrowMut;
use std::time::Duration;

use homeserver_receive_process::{Command, CommandBuilder};
use serenity::builder::CreateMessage;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

use crate::bot_config::ConfigContainer;

use super::minecraft::SystemctlCommand;
use super::EmbedMessageBuilder;

const REQUEST_TIMEOUT: u64 = 15;

#[group]
#[commands(start, status, stop, restart)]
#[prefixes("terraria", "tr")]
#[description = "Terraria管理コマンド"]
pub struct Terraria;

impl Terraria {
    async fn generate_url(ctx: &Context) -> String {
        let data_read = ctx.data.read().await;
        let config = data_read
            .get::<ConfigContainer>()
            .expect("Expected ConfigContainer in TypeMap");

        let address = config.address();
        format!(
            "http://{}:{}/terraria",
            address.home_server_ip(),
            address.home_server_port()
        )
    }

    async fn terraria_command_exec(
        command: SystemctlCommand,
        ctx: &Context,
        msg: &Message,
    ) -> CommandResult {
        let typing = msg.channel_id.start_typing(&ctx.http).unwrap();
        let data_read = ctx.data.read().await;
        let url = Terraria::generate_url(ctx).await;
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
                        ctx.set_activity(Activity::playing("Terraria")).await
                    }
                    SystemctlCommand::Stop => ctx.reset_presence().await,
                    _ => (),
                }

                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.set_embed({
                            let mut e = EmbedMessageBuilder::default()
                                .success(success)
                                .message(body)
                                .build();

                            // when exec start, show message Terraria password
                            if let SystemctlCommand::Start = command {
                                let game_info = data_read
                                    .get::<ConfigContainer>()
                                    .expect("Expected ConfigContainer in TypeMap")
                                    .gameinfo();
                                if let Some(game_info) = game_info {
                                    e.field(
                                        "[Terraria pass info]",
                                        game_info
                                            .sdtd_password()
                                            .as_ref()
                                            .unwrap_or(&String::new()),
                                        false,
                                    );
                                }
                            }

                            e
                        })
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

#[command]
#[description = "Terrariaサーバを起動する"]
async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Terraria::terraria_command_exec(SystemctlCommand::Start, ctx, msg).await
}

#[command]
#[description = "Terrariaサーバの状態を表示する"]
async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Terraria::terraria_command_exec(SystemctlCommand::Status, ctx, msg).await
}

#[command]
#[description = "Terrariaサーバを停止する"]
async fn stop(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Terraria::terraria_command_exec(SystemctlCommand::Stop, ctx, msg).await
}

#[command]
#[description = "Terrariaサーバを再起動する"]
async fn restart(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Terraria::terraria_command_exec(SystemctlCommand::Restart, ctx, msg).await
}
