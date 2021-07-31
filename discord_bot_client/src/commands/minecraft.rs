use homeserver_receive_process::{Command, CommandBuilder};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::bot_config::ConfigContainer;

#[group]
#[commands(start)]
#[prefixes("minecraft", "mc")]
#[description = "Minecraft管理コマンド"]
pub struct Minecraft;

#[command]
pub async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let home_server_ip_port = {
        let data_read = ctx.data.read().await;
        let config = data_read
            .get::<ConfigContainer>()
            .expect("Expected ConfigContainer in TypeMap");

        format!(
            "http://{}:{}/minecraft",
            config.address().home_server_ip(),
            config.address().home_server_port()
        )
    };
    let post_data = CommandBuilder::default()
        .request("start".to_string())
        .administrator(false)
        .build()
        .unwrap();

    let client = reqwest::Client::new();
    let response = client
        .post(home_server_ip_port)
        .json(&post_data)
        .send()
        .await
        .unwrap();

    let body = response.text().await.unwrap();
    msg.channel_id.say(&ctx.http, &body).await?;

    Ok(())
}
