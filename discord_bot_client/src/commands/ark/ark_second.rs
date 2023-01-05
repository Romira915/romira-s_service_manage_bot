use std::time::Duration;

use homeserver_receive_process::CommandBuilder;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::bot_config::ConfigContainer;
use crate::commands::minecraft::{self, SystemctlCommand};
use crate::commands::EmbedMessageBuilder;

const REQUEST_TIMEOUT: u64 = 30;

#[group]
#[commands(start, status, stop, restart)]
#[prefixes("second")]
#[description = "ARK second server管理コマンド"]
pub struct ArkSecond;

impl ArkSecond {
    pub(crate) async fn generate_url(ctx: &Context) -> String {
        let data_read = ctx.data.read().await;
        let config = data_read
            .get::<ConfigContainer>()
            .expect("Expected ConfigContainer in TypeMap");

        let address = config.address();
        format!("https://{}/ark-second", address.home_server_ip(),)
    }

    pub(crate) async fn ark_command_exec(
        command: SystemctlCommand,
        ctx: &Context,
        msg: &Message,
    ) -> CommandResult {
        let typing = msg.channel_id.start_typing(&ctx.http).unwrap();
        let data_read = ctx.data.read().await;
        let url = ArkSecond::generate_url(ctx).await;
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
                        ctx.set_activity(Activity::playing("ARK")).await
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
                                        "[ARK pass info]",
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
#[description = "ARKサーバを起動する"]
async fn start(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkSecond::ark_command_exec(SystemctlCommand::Start, ctx, msg).await
}

#[command]
#[description = "ARKサーバの状態を表示する"]
async fn status(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkSecond::ark_command_exec(SystemctlCommand::Status, ctx, msg).await
}

#[command]
#[description = "ARKサーバを停止する"]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkSecond::ark_command_exec(SystemctlCommand::Stop, ctx, msg).await
}

#[command]
#[description = "ARKサーバを再起動する"]
async fn restart(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ArkSecond::ark_command_exec(SystemctlCommand::Restart, ctx, msg).await
}
