pub mod bot_config;
pub mod commands;

use std::collections::HashSet;

use commands::{
    conversation::{ai_chan, dousite, hamu, nannnoimiga, otu, what, www, yosi},
    simple::*,
};
use log::{debug, error, info, LevelFilter};

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

        if msg.content.starts_with("/") && msg.content.split_whitespace().count() == 2 {
            if let Err(why)  = msg.channel_id.send_message(&ctx.http, |m|{
                m.embed(|e| {
                    e.title("コマンドを実行しようとしてる？")
                    .description("`/` プレフィックスは無効になりました．\n今後は `~` プレフィックスを使用してください．\n\n詳しくは `~help` で参照できます．")
                    .colour(Colour::ORANGE)
                })
            }).await {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content.starts_with("草") || {
            let len = msg.content.chars().count();
            let mut www = msg.content.clone();
            www.retain(|f| f == 'w');
            let www_len = www.chars().count();

            www_len as f32 / len as f32 > 50.0
        } {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(www()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if (msg.content.contains("ヨシ") || msg.content.contains("ﾖｼ"))
            && (msg.content.contains("！") || msg.content.contains("!"))
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(yosi()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content.starts_with("?") || msg.content.starts_with("？") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(what()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content.contains("どうして") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(dousite()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        // Ai chan reply
        if msg.content.contains("あいちゃん") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(ai_chan()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content.contains("おつかれ") || msg.content.contains("お疲れ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(otu()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content.contains("酒") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(otu()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content.starts_with("あっ") || msg.content.starts_with("やべ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(nannnoimiga()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content.contains("ハムうめぇ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(hamu()))
                .await
            {
                error!("Error sending message: {:?}", why);
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
