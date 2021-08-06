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

use super::EmbedMessageBuilder;

const REQUEST_TIMEOUT: u64 = 5;

#[group]
#[commands(start, status)]
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
}

#[command]
pub async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = Minecraft::generate_url(ctx).await;

    let post_data = CommandBuilder::default()
        .request("start".to_string())
        .administrator(false)
        .build()
        .unwrap();

    let client = reqwest::Client::new();
    let response = client.post(url).json(&post_data).send().await.unwrap();

    let body = response.text().await.unwrap();
    msg.channel_id.say(&ctx.http, &body).await?;

    Ok(())
}

#[command]
pub async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();
    let url = Minecraft::generate_url(ctx).await;

    let post_data = CommandBuilder::default()
        .request("status".to_string())
        .administrator(false)
        .build()
        .unwrap();

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&post_data)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .send()
        .await;

    match response {
        Ok(res) => {
            let body = res.text().await.unwrap();
            msg.channel_id.say(&ctx.http, &body).await?;
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
                .await
                .unwrap();
        }
    }

    typing.stop();

    Ok(())
}
