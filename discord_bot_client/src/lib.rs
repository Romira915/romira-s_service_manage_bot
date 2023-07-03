pub mod bot_config;
pub mod commands;
pub mod models;
pub mod state;

use std::collections::HashSet;

use bot_config::ConfigContainer;
use commands::{conversation::*, simple::*};
use log::{debug, error, info};

use rand::{distributions::WeightedIndex, prelude::*};
use regex::Regex;
use serde_json::{json, Value};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, event::ResumedEvent, id::UserId, prelude::Ready},
    utils::Colour,
};
use state::{BotState, BotStateContainer};

const SONNEKI_INTERVAL_MS: u64 = 1000;
const RINNA_CCE_ENDPOINT: &str = "https://api.rinna.co.jp/models/cce";

#[group]
#[commands(ping)]
pub struct GeneralT;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        info!("Resumed")
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let content = msg.content.clone();
        let regex: Regex = Regex::new("<@\\d+>").unwrap();
        let tmp = regex.replace_all(&content, "");
        let content_without_mentions = tmp.trim_start();

        // Read Config Block
        {
            let data_read = ctx.data.read().await;
            let config = data_read
                .get::<ConfigContainer>()
                .expect("Expected ConfigContainer in TypeMap");
            // コマンドなら早期リターン
            if content.starts_with(config.discord().prefix()) {
                return;
            }

            if content.starts_with('/') && content.split_whitespace().count() == 2 {
                if let Err(why)  = msg.channel_id.send_message(&ctx.http, |m|{
                    m.embed(|e| {
                        e.title("コマンドを実行しようとしてる？")
                        .description(&format!("`/` プレフィックスは無効になりました．\n今後は `{}` プレフィックスを使用してください．\n\n詳しくは `~help` で参照できます．",config.discord().prefix()))
                        .colour(Colour::ORANGE)
                    })
                }).await {
                    error!("Error sending message: {:?}", why);
                }
            }
        }

        // Bot Quiet On/Off
        if msg.mentions_me(&ctx.http).await.unwrap() {
            let mut data_write = ctx.data.write().await;
            let bot_state = data_write
                .get_mut::<BotStateContainer>()
                .expect("Failed to ctx data write BotState");

            if content_without_mentions == "だまれ" {
                bot_state.is_quiet = true;
                log::info!("is_quiet {}", bot_state.is_quiet);
                if let Err(why) = msg
                    .channel_id
                    .send_message(&ctx.http, |m| m.content("だまる :anya19:"))
                    .await
                {
                    error!("Error sending message: {:?}", why);
                }

                return;
            }

            if content_without_mentions == "ヨシ" {
                bot_state.is_quiet = false;
                log::info!("is_quiet {}", bot_state.is_quiet);
                if let Err(why) = msg
                    .channel_id
                    .send_message(&ctx.http, |m| m.content("しゃべる :ANYA:"))
                    .await
                {
                    error!("Error sending message: {:?}", why);
                }

                return;
            }
        }

        // 会話AI
        {
            let choices = [true, false];
            let weights = [0.0005, 0.9995];
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();

            let data_read = ctx.data.read().await;
            let bot_state = data_read
                .get::<BotStateContainer>()
                .expect("Failed to ctx data read BotState");

            if !bot_state.is_quiet
                && (msg.mentions_me(&ctx.http).await.unwrap() || choices[dist.sample(&mut rng)])
            {
                log::info!("Hit 会話AI");
                let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

                let config = data_read
                    .get::<ConfigContainer>()
                    .expect("Expected ConfigContainer in TypeMap");

                let client = reqwest::Client::new();
                log::info!("Request to CCE");
                let resp = client
                    .post(RINNA_CCE_ENDPOINT)
                    .header("content-type", "application/json")
                    .header("cache-control", "no-cache")
                    .header(
                        "Ocp-Apim-Subscription-Key",
                        config.secret().rinna_cce_subscription_key(),
                    )
                    .json(&json!({
                        "rawInput": format!("B: {}A:", content_without_mentions),
                        "outputLength": 25
                    }))
                    .send()
                    .await
                    .unwrap();

                log::info!("CCE Status code {}", resp.status().as_str());
                if resp.status().is_success() {
                    let json: Value = resp.json().await.unwrap();
                    let answer = json.get("answer").unwrap().as_str().unwrap();

                    if let Err(why) = msg
                        .channel_id
                        .send_message(&ctx.http, |m| m.content(&answer))
                        .await
                    {
                        error!("Error sending message: {:?}", why);
                    };

                    typing.stop().unwrap_or_default();

                    return;
                }

                typing.stop().unwrap_or_default();
            }
        }

        {
            let data_read = ctx.data.read().await;
            let bot_state = data_read
                .get::<BotStateContainer>()
                .expect("Failed to ctx data read BotState");
            if !bot_state.is_quiet {
                // リアクション
                conversation(&ctx, &msg).await;
            }
        }
    }
}

#[help]
#[individual_command_tip = "~ Welcome to ServerManage Help! ~\n\n\
特定のコマンドに関する詳細情報が必要な場合は，コマンドを引数として渡してください．"]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    debug!("help commands");
    Ok(())
}
