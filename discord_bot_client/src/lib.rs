pub mod bot_config;
pub mod commands;

use std::{collections::HashSet, time::Duration};

use commands::{
    conversation::{
        ai_chan, akeome, buy_kyan, chiyopanchi, dontstop, exactly, fight_anya, hadou, hamu,
        hello_anya, hello_tenjyo, hopak, hugu, ikare, ikare_one, imwin, kakusensou, konata,
        lose_syamiko, meat_syamiko, monhanneko, motidesuwa, motyo, mun, nannnoimiga, otu, pakupaku,
        paxan, pita, punch_anya, safety, sake, soturon_owata, souhayarann, tearai, teio_tuntun,
        thesis_donot_end, tiyono_o_, today_ganba, wakannnaippi, wakuwaku, wara_anya, what_buru,
        world_end, yada, yosi_inoti, yosi_three, DOUSITE_EMBEDS, KUSA, NAMEURARA_EMBEDS,
        SONNEKINEKO_EMBEDS, TENJYO_EMBEDS, WHAT_EMBEDS, YOSI_EMBEDS,
    },
    simple::*,
};
use log::{debug, error, info};

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
    model::{channel::Message, event::ResumedEvent, id::UserId, prelude::Ready},
    utils::Colour,
};

use tokio::time;

const SONNEKI_INTERVAL_MS: u64 = 1000;

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

        if content.contains("ヨシヨシヨシ") || content.contains("ヨシ！ヨシ！ヨシ！")
        {
            if let Err(why) = msg
                .channel_id
                // .send_message(&ctx.http, |m| m.set_embed(yosiyosiyosi()))
                .send_message(&ctx.http, |m| m.set_embed(yosi_three()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        } else if (content.contains("ヨシ") || content.contains("ﾖｼ"))
            && (content.contains("！") || content.contains("!"))
        {
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();

            let yosi_embeds_added_probability = {
                let prob = vec![0.7, 0.3];
                prob.into_iter()
                    .zip(YOSI_EMBEDS)
                    .collect::<Vec<(f64, fn() -> CreateEmbed)>>()
            };
            let embed = yosi_embeds_added_probability
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

        if content.starts_with("?") || content.starts_with("？") || content.starts_with("は？") {
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();

            let yosi_embeds_added_probability = {
                let prob = vec![0.1, 0.9];
                prob.into_iter()
                    .zip(WHAT_EMBEDS)
                    .collect::<Vec<(f64, fn() -> CreateEmbed)>>()
            };
            let embed = yosi_embeds_added_probability
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

        if content.contains("どうして") {
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();

            let yosi_embeds_added_probability = {
                let prob = vec![0.3, 0.7];
                prob.into_iter()
                    .zip(DOUSITE_EMBEDS)
                    .collect::<Vec<(f64, fn() -> CreateEmbed)>>()
            };
            let embed = yosi_embeds_added_probability
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

        // Ai chan reply
        if content.eq("あいちゃん")
            || content.eq("Aiちゃん")
            || content.contains("あいちゃんが静かでヨシ")
        {
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

        if content.contains("( ˶ˆᴗˆ˶ )") {
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
            || content.contains("Hello tenjyo")
            || content.contains("Hello tenjo")
            || content.contains("hello tenjyo")
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

        if content.contains("ｼﾃ…ｺﾛｼﾃ……")
            || content.contains("コロシテ")
            || content.contains("侮辱")
            || content == "ヨシ"
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(yosi_inoti()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("楽しい")
            || content.contains("ワクワク")
            || content.contains("わくわく")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(wakuwaku()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("おはよう") || content.contains("おはやいます") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(hello_anya()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("だいじょぶ")
            || content.contains("だいじょうぶ")
            || content.contains("大丈夫")
            || content.contains("がんば")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(fight_anya()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("笑") || content.contains("ﾌｯ") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(wara_anya()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("うるせぇ")
            || content.contains("うるせえ")
            || content.contains("パンチ")
            || content.contains("ブチ切れ")
            || content.contains("死ぬ")
            || content.contains("死ね")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(punch_anya()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("止まる") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(dontstop()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("わからない")
            || content.contains("わかんない")
            || content.contains("わからん")
            || content.contains("わかんね")
            || content.contains("分からない")
            || content.contains("分かんない")
            || content.contains("分からん")
            || content.contains("分かんね")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(wakannnaippi()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("かくせんそう") || content.contains("核") || content.contains("戦争")
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(kakusensou()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if (content.contains("世界") && content.contains("せかい"))
            || (content.contains("終わ") && content.contains("おわ"))
        {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(world_end()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("買っちった") || content.contains("かっちった") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(buy_kyan()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("勝っ") || content.contains("勝つ") || content.contains("負け") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(lose_syamiko()))
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if content.contains("焼肉") || content.contains("バァァーン") {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(meat_syamiko()))
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

        time::sleep(Duration::from_millis(SONNEKI_INTERVAL_MS)).await;
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

        time::sleep(Duration::from_millis(SONNEKI_INTERVAL_MS)).await;
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
