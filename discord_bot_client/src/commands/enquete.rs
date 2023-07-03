use std::ffi::CString;

use image::DynamicImage;
use nix::unistd::{self, ForkResult};
use serde_json::json;
use serde_json::Value;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::Context;
use serenity::prelude::*;

use crate::bot_config::ConfigContainer;
use crate::models::ChatGPTResponse;
use crate::models::ChatGPTSchema;

const PROMPT_ENDPOINT: &str =
    "https://k5vi72fcdo5u6gjqmuaqu5yoba0draxm.lambda-url.ap-northeast-1.on.aws/prompt";
const RINNA_ENDPOINT: &str = "https://api.rinna.co.jp/models/tti/v2";
const OPENAI_ENDPOINT: &'static str =
    "https://genbaneko-ai.openai.azure.com/openai/deployments/bocchi/completions";
const OPENAI_QUERY: [(&'static str, &'static str); 1] = [("api-version", "2022-12-01")];

#[group]
#[commands(seek)]
#[description = "アンケート・フォーム用コマンド"]
pub struct Enquete;

#[command]
pub async fn seek(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.message();
    log::debug!("arg {:?}", arg);
    log::info!("args {}", args.rest());

    let mention_user_id = msg
        .mentions
        .iter()
        .map(|u| u.mention().to_string())
        .collect::<Vec<String>>();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.allowed_mentions(|am| am.replied_user(true))
                .content(&format!("remaind {}", mention_user_id.join(" ")))
        })
        .await
        .unwrap();

    msg.delete(&ctx.http).await.unwrap();

    Ok(())
}
