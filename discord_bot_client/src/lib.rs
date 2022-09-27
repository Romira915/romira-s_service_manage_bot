pub mod bot_config;
pub mod commands;

use std::collections::HashSet;

use bot_config::ConfigContainer;
use commands::{conversation::*, simple::*};
use log::{debug, error, info};

use rand::{distributions::WeightedIndex, prelude::*};
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

const SONNEKI_INTERVAL_MS: u64 = 1000;
const RINNA_CCE_ENDPOINT: &str = "https://api.rinna.co.jp/models/cce";

#[group]
#[commands(ping)]
pub struct General;

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

        let data_read = ctx.data.read().await;
        let config = data_read
            .get::<ConfigContainer>()
            .expect("Expected ConfigContainer in TypeMap");

        let content = msg.content.clone();

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

        // 会話AI
        {
            let choices = [true, false];
            let weights = [0.20, 0.80];
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();

            if msg.mentions_me(&ctx.http).await.unwrap() || choices[dist.sample(&mut rng)] {
                log::info!("Hit 会話AI");
                let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

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
                        "rawInput": format!("B: {}A:",&content),
                        "outputLength": 30
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

                    typing.stop();

                    return;
                }

                typing.stop();
            }
        }

        conversation(&ctx, &msg).await;
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
