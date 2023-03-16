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
#[commands(draw, draw_jp, bocchi)]
#[prefixes("ai", "ml")]
#[description = "機械学習コマンド"]
pub struct Ai;

#[command]
pub async fn draw(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.message();
    log::info!("{:?}", arg);
    if arg.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("引数を与えてね")
                        .field("Example", "~ai draw cat", false)
                })
            })
            .await?;

        return Ok(());
    }

    let client = reqwest::Client::new();

    let prompt: Value = client.get(PROMPT_ENDPOINT).send().await?.json().await?;
    if prompt["prompt"] != "~completed" {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| e.title("前回の絵をまだ描いているが？"))
            })
            .await?;

        return Ok(());
    }

    client
        .post(PROMPT_ENDPOINT)
        .json(&json!({ "prompt": arg }))
        .send()
        .await?;

    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            log::debug!("Main({}) forked a child({})", unistd::getpid(), child);
        }
        Ok(ForkResult::Child) => {
            log::debug!(
                "Child({}) started. PPID is {}",
                unistd::getpid(),
                unistd::getppid()
            );
            let data_read = ctx.data.read().await;
            let config = data_read
                .get::<ConfigContainer>()
                .expect("Expected ConfigContainer in TypeMap");

            let python_path = CString::new(config.path().python().as_str()).unwrap();
            let env = [
                CString::new(format!("DRIVER_PATH={}", config.path().geckodriver_path())).unwrap(),
                CString::new(format!(
                    "FIREFOX_PROFILE={}",
                    config.path().firefox_profile()
                ))
                .unwrap(),
            ];

            if let Ok(_) = unistd::execve(
                python_path.as_c_str(),
                &[
                    python_path.as_c_str(),
                    CString::new("./colab-cli/main.py").unwrap().as_c_str(),
                ],
                &env,
            ) {}
        }
        Err(_) => log::debug!("Fork failed"),
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("絵を描くよ～(o・∇・o)")
                    .description("3分くらいで描き終わるよ～(o・∇・o)")
            })
        })
        .await?;

    Ok(())
}

#[command]
pub async fn draw_jp(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.message();
    let arg = arg.trim_start_matches('\"').trim_end_matches('\"');
    log::info!("{:?}", arg);
    if arg.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("引数を与えてね")
                        .field("Example", "~ai draw_jp cat", false)
                })
            })
            .await?;

        return Ok(());
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("絵を描くよ～(o・∇・o)")
                    .description("20秒くらいで描き終わるよ～(o・∇・o)")
            })
        })
        .await?;
    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

    let data_read = ctx.data.read().await;
    let config = data_read
        .get::<ConfigContainer>()
        .expect("Expected ConfigContainer in TypeMap");

    let client = reqwest::Client::new();
    let resp = client
        .post(RINNA_ENDPOINT)
        .header("content-type", "application/json")
        .header("cache-control", "no-cache")
        .header(
            "Ocp-Apim-Subscription-Key",
            config.secret().rinna_subscription_key(),
        )
        .json(&json!({
            "prompts": arg,
            "scale": 7.5
        }
        ))
        .send()
        .await
        .unwrap();

    log::info!("status code {}", resp.status().as_str());
    // status code 500
    if resp.status() == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!(
                        "Status code {} Internal Server Error",
                        resp.status().as_str()
                    ))
                    .description("暫く時間をおけ")
                })
            })
            .await?;

        return Ok(());
    }
    let json: Value = resp.json().await.unwrap();

    // TODO: as_strで""でないかも
    let split: Vec<String> = json["image"]
        .to_string()
        .splitn(2, ',')
        .map(|i| i.to_string())
        .collect();

    // let split: Vec<String> = image.splitn(2, ",").map(|i| i.to_string()).collect();

    let (_format, image_base64) = (
        split[0].as_str(),
        split[1].as_str().trim().trim_end_matches('\"'),
    );
    let image_raw = base64::decode_config(image_base64, base64::STANDARD).unwrap();

    let dynamic_image = image::load_from_memory(&image_raw).unwrap();
    match dynamic_image {
        DynamicImage::ImageRgb8(image) => image.save("ai-draw-jp.png").unwrap(),
        _ => (),
    }
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.image("attachment://ai-draw-jp.png"))
                .add_file("ai-draw-jp.png")
        })
        .await?;
    typing.stop().unwrap();

    Ok(())
}

#[command]
pub async fn bocchi(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.message();
    log::info!("{:?}", arg);
    if arg.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("引数を与えてね")
                        .field("Example", "~ai bocchi ぼっちちゃん！", false)
                })
            })
            .await?;

        return Ok(());
    }

    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

    let data_read = ctx.data.read().await;
    let config = data_read
        .get::<ConfigContainer>()
        .expect("Expected ConfigContainer in TypeMap");

    let mut chat_gpt_data = ChatGPTSchema::new_with_bocchi(0.5, 0.95, 0, 0, 2500);
    chat_gpt_data.user_query(arg);

    let client = reqwest::Client::new();
    let response = client
        .post(OPENAI_ENDPOINT)
        .query(&OPENAI_QUERY)
        .header("Content-Type", "application/json")
        .header("api-key", config.secret().openai_api_key())
        .json(&chat_gpt_data)
        .send()
        .await?;

    let chat_gpt_res = response.json::<ChatGPTResponse>().await?;

    msg.channel_id
        .say(&ctx.http, &chat_gpt_res.choices[0].text)
        .await?;

    typing.stop().unwrap_or_default();

    Ok(())
}
