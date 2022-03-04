pub mod bot_config;
pub mod commands;

use std::{collections::HashSet, time::Duration};

use commands::{
    conversation::{
        ai_chan, akeome, chiyopanchi, dousite, exactly, hadou, hamu, hello_tenjyo, hopak, hugu,
        ikare, ikare_one, imwin, konata, kusadora0, kusadora1, monhanneko, motidesuwa, motyo, mun,
        nannnoimiga, otu, pakupaku, paxan, pita, safety, sake, soturon_owata, souhayarann, tearai,
        teio_tuntun, thesis_donot_end, tiyono_o_, today_ganba, tyuuname, what, what_buru, www,
        yada, yosi, KUSA, NAMEURARA_EMBEDS, SONNEKINEKO_EMBEDS, TENJYO_EMBEDS,
    },
    simple::*,
};
use log::{debug, error, info, LevelFilter};

use rand::prelude::*;
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::{Context, EventHandler},
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{
        channel::Message,
        event::ResumedEvent,
        id::UserId,
        prelude::{Activity, Ready},
    },
    utils::Colour,
};

use tokio::time;

const sonneki_interval_ms: u64 = 1000;

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

        if content.ends_with("草")
            || {
                let len = content.chars().count();
                let mut www = content.clone();
                www.retain(|f| f == 'w' || f == 'ｗ');
                let www_len = www.chars().count();

                www_len as f32 / len as f32 > 0.5
            }
            || content.ends_with("www")
        {
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();
            let kusa_embed = KUSA.choose(&mut rng).unwrap();

            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(kusa_embed()))
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
        if content.eq("あいちゃん") || content.eq("Aiちゃん") {
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
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();

            let tenjyo_embeds_added_probability = {
                let prob = vec![0.8, 0.2];
                prob.into_iter()
                    .zip(TENJYO_EMBEDS)
                    .collect::<Vec<(f64, fn() -> CreateEmbed)>>()
            };
            let embed = tenjyo_embeds_added_probability
                .choose_weighted(&mut rng, |item| item.0)
                .unwrap()
                .1;

            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(embed()))
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

        if content.contains("ぱくぱく") || content.contains("パクパク") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(pakupaku()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.ends_with("今日も一日") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(today_ganba()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.ends_with("😭") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.content(":sob:"))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.ends_with("・ｖ・") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(konata()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.starts_with("( ˶ˆᴗˆ˶ )") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(teio_tuntun()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("コサックダンス")
            || content.contains("こさっくだんす")
            || content.contains("ホパーク")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(hopak()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("やだ")
            || content.contains("いやだ")
            || content.contains("嫌")
            || content.contains("イヤ")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(yada()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("波動") || content.contains("はどう") || content.contains("昇龍拳")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(hadou()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("ぴた") || content.contains("ピタ") || content.contains("ヒミツ")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(pita()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("あけおめ")
            || content.contains("あけましておめでとう")
            || content.contains("明けましておめでとう")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(akeome()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("餅")
            || content.contains("もちですわ")
            || content.contains("もちうめぇ")
            || content.contains("おしるこ")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(motidesuwa()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("損") || content.contains("くそったれ") || content.contains("デデーン")
        {
            sonnekineko(&ctx, &msg).await;
        }

        if content.contains("パァン") || content.contains("ぱぁん") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(paxan()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("パンチ")
            || content.contains("かわいい")
            || content.contains("カワイイ")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(chiyopanchi()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("なめる") || content.contains("なめてる") || content.contains("舐め")
        {
            namebetu(&ctx, &msg).await;
        }

        if content.contains("Hello Tenjyo")
            || content.contains("はろーてんじょう")
            || content.contains("ハロー")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(hello_tenjyo()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("卒論おわら") || content.contains("卒論終わら") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(thesis_donot_end()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("そのとおり") || content.contains("その通り") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(exactly()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("ファイト") || content.contains("ふぁいと") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(tiyono_o_()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("何") && (content.contains("?") || content.contains("？")) {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(what_buru()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("卒論")
            && (content.contains("終わった") || content.contains("おわった"))
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(soturon_owata()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("俺の勝ち") || content.contains("おれのかち") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(imwin()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("あぶな")
            || content.contains("危な")
            || content.contains("安全")
            || content.contains("あんぜん")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(safety()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if (content.contains("モンハン") || content.contains("もんはん"))
            && content.contains("やろ")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(monhanneko()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("(o・∇・o)")
            || content.contains("終わり")
            || content.contains("おわり")
            || content.contains("もちょだよ")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(motyo()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }
    }
}

async fn sonnekineko(ctx: &Context, msg: &Message) {
    for embed in &SONNEKINEKO_EMBEDS {
        if let Err(why) = msg
            .channel_id
            .send_message(&ctx.http, |m| m.set_embed(embed()))
            .await
        {
            error!("Error sending message: {:?}", why);
        }

        time::sleep(Duration::from_millis(sonneki_interval_ms)).await;
    }
}

async fn namebetu(ctx: &Context, msg: &Message) {
    for embed in &NAMEURARA_EMBEDS {
        if let Err(why) = msg
            .channel_id
            .send_message(&ctx.http, |m| m.set_embed(embed()))
            .await
        {
            error!("Error sending message: {:?}", why);
        }

        time::sleep(Duration::from_millis(sonneki_interval_ms)).await;
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
