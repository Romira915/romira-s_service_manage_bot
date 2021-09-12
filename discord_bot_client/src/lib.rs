pub mod bot_config;
pub mod commands;

use std::collections::HashSet;

use commands::{
    conversation::{
        ai_chan, dousite, hamu, hugu, ikare, ikare_one, mun, nannnoimiga, otu, sake, souhayarann,
        tearai, tenjou, what, www, yosi,
    },
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

        let content = msg.content.clone();

        if content.starts_with("/") && content.split_whitespace().count() == 2 {
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

        if content.ends_with("草") || {
            let len = content.chars().count();
            let mut www = content.clone();
            www.retain(|f| f == 'w' || f == 'ｗ');
            let www_len = www.chars().count();

            www_len as f32 / len as f32 > 0.5
        } {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(www()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if (content.contains("ヨシ") || content.contains("ﾖｼ"))
            && (content.contains("！") || content.contains("!"))
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(yosi()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.starts_with("?") || content.starts_with("？") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(what()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("どうして") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(dousite()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        // Ai chan reply
        if content.starts_with("あいちゃん") || content.starts_with("Aiちゃん") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(ai_chan()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("おつかれ") || content.contains("お疲れ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(otu()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("酒") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(sake()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.starts_with("あっ") || content.starts_with("やべ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(nannnoimiga()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("ハムうめぇ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(hamu()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content == "(☝\u{fe0f} ՞ਊ ՞)☝\u{fe0f}" || content == "(☝ ՞ਊ ՞)☝" {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(ikare_one()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("(☝\u{fe0f} ՞ਊ ՞)☝\u{fe0f}") || content.contains("(☝ ՞ਊ ՞)☝(☝ ՞ਊ ՞)☝")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.content(&content).set_embed(ikare()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("そうはならんやろ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(souhayarann()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("てんじょう") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(tenjou()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("ふぐ") || content.contains("フグ") || content.contains("🐡") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(hugu()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("えい、えい") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(mun()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("ただいま") || content.contains("帰った") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(tearai()))
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
