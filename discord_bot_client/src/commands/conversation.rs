use chrono::{Duration, Utc};
use serenity::builder::CreateEmbed;

pub const KUSA: [fn() -> CreateEmbed; 3] = [www, kusadora0, kusadora1];
pub const SONNEKINEKO_EMBEDS: [fn() -> CreateEmbed; 4] =
    [sonnekineko0, sonnekineko1, sonnekineko2, sonnekineko3];
pub const NAMEURARA_EMBEDS: [fn() -> CreateEmbed; 2] = [tyuuname, urabetu];
pub const TENJYO_EMBEDS: [fn() -> CreateEmbed; 2] = [tenjyo, hello_tenjyo];

pub fn www() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/profile_images/879510459027562496/jA8ScZnS_400x400.jpg");

    embed
}

pub fn yosi() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("ヨシ！")
        .image("https://automaton-media.com/wp-content/uploads/2020/03/20200304-115748-header.jpg");

    embed
}

pub fn what() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("http://e-village.main.jp/gazou/image_gazou/gazou_0187.jpg");

    embed
}

pub fn dousite() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("どうして")
        .image("https://i.pinimg.com/564x/d3/75/8d/d3758dd7baa347b799d457c5306cd294.jpg");

    embed
}

pub fn ai_chan() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("よくわかったね\n君は今日からヨシ！")
        .image("https://matome.hacker-hacker.com/wp-content/uploads/2020/05/genba-1.gif");

    embed
}

pub fn otu() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://i.pinimg.com/originals/16/c2/c6/16c2c6c4b46b06920b6ef7dc9dd3f47f.jpg");

    embed
}

pub fn sake() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://up.gc-img.net/post_img/2020/02/NJ7qIt2WJyy1ymo_fnJcL_24.jpeg");

    embed
}

pub fn nannnoimiga() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://livedoor.blogimg.jp/mhworld_pc-y4hufkzc/imgs/6/8/68c0d368.gif");

    embed
}

pub fn hamu() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://livedoor.blogimg.jp/mhworld_pc-y4hufkzc/imgs/9/1/910434f4.jpg");

    embed
}

pub fn ikare_one() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("(☝ ՞ਊ ՞）☝")
        .description("参考文献")
        .url("https://is.gd/Pu9FA8");

    embed
}

pub fn ikare() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("(☝ ՞ਊ ՞）☝")
        .image("https://kai-you.net/images/a/2016/12/30c99b2f0472631f8669ecdf17c7ca0e.jpg");

    embed
}

pub fn souhayarann() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("なっとるやろがい！！")
        .image("https://assets.st-note.com/production/uploads/images/13219811/picture_pc_a39235ada75c9200b1d1ae689e731630.jpg?width=800");

    embed
}

pub fn tenjyo() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("もしかして: ")
        .url("https://www.google.com/search?q=%E3%81%A6%E3%82%93%E3%81%98%E3%82%87%E3%81%86")
        .image("http://www.tamura-kensetsu.com/tkblog/photo/yabudukanoiegoutenjyoubefre.jpg");

    embed
}

pub fn hugu() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://image.itmedia.co.jp/nl/articles/1903/25/l_miya_1903mizuhakuhugu02.jpg");

    embed
}

pub fn mun() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("むん！")
        .image("https://livedoor.blogimg.jp/akb83-c9npozlg/imgs/c/4/c400e113.jpg");

    embed
}

pub fn tearai() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("おかえり")
        .image("https://up.gc-img.net/post_img/2020/02/NJ7qIt2WJyy1ymo_6MqS8_1511.jpeg");

    embed
}

pub fn pakupaku() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("パクパクですわ")
        .image("https://objectstorage.ap-tokyo-1.oraclecloud.com/n/nr7eduszgfzb/b/image-bucket/o/bot%2Fe049941ecd6b2c435bd3ee3f44a42a8c.jpg");

    embed
}

pub fn today_ganba() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("ぞいっていえ")
        .image("https://pbs.twimg.com/media/BoM6gddCQAAl_SB.jpg");

    embed
}

pub fn konata() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title("↑これこなた").description("参考文献").url(
        "https://discord.com/channels/617069702983843871/822758393617186826/886768085909008405",
    );

    embed
}

pub fn teio_tuntun() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://c.tenor.com/Dmk6LMwMDcMAAAAC/%E3%83%88%E3%82%A6%E3%82%AB%E3%82%A4%E3%83%86%E3%82%A4%E3%82%AA%E3%83%BC-%E3%83%80%E3%83%B3%E3%82%B9.gif");

    embed
}

pub fn hopak() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://objectstorage.ap-tokyo-1.oraclecloud.com/n/nr7eduszgfzb/b/image-bucket/o/bot%2Fkosakkudannsu-umamusume.gif");

    embed
}

pub fn yada() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://umamusu.more-gamer.com/wp-content/uploads/2021/07/qATOki7.gif");

    embed
}

pub fn hadou() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://ウマ娘プラス.com/wp-content/uploads/2018/06/ezgif-6-146909d34c.gif");

    embed
}

pub fn pita() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://xn--gckvb3e1a0dy660b.com/wp-content/uploads/2018/06/Uma-Musume-Gif-2.gif");

    embed
}

pub fn akeome() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/media/EqjcEy_UwAERsja.jpg");

    embed
}

pub fn motidesuwa() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("もちですわ")
        .author(|a| {
            a.name("pixiv")
                .url("https://www.pixiv.net/artworks/95212164")
        })
        .url("https://twitter.com/katwo_1/status/1477190523538784256")
        .image("https://pbs.twimg.com/media/FIAI8xjagAI2l9Z?format=jpg&name=large");

    embed
}

pub fn kusadora0() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/media/DIETxlPUQAEn9HD?format=jpg&name=900x900");

    embed
}

pub fn kusadora1() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/media/DIETxlTUIAIsWXL?format=jpg&name=large");

    embed
}

pub fn sonnekineko0() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/media/FJYrAmBaQAAUZCo?format=jpg&name=large");

    embed
}

pub fn sonnekineko1() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/media/FJYrAl-agAAkuV_?format=jpg&name=large");

    embed
}

pub fn sonnekineko2() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/media/FJYrAmCagAUcO58?format=jpg&name=large");

    embed
}

pub fn sonnekineko3() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("デデーン！")
        .image("https://pbs.twimg.com/media/FJYrAmZagAAd-W2?format=jpg&name=large");

    embed
}

pub fn paxan() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    let utc_date = Utc::today();
    let jst_date = utc_date + Duration::hours(9);
    let season = {
        let month = jst_date.format("%m").to_string().parse::<u32>().unwrap();
        match month {
            3..=5 => "春",
            6..=8 => "夏",
            9..=11 => "秋",
            12 | 1..=2 => "冬",
            _ => "何",
        }
    };
    embed
        .title(format!("{}のパァン祭", season))
        .image("https://pbs.twimg.com/media/ETUdDbVU0AAiKh4.jpg");

    embed
}

pub fn chiyopanchi() -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    embed
        .title("チヨパンチ！")
        .footer(|f|f.text("ﾁﾖﾁﾖ"))
        .image("https://xn--o9j0bk9l4k169rk1cxv4aci7a739c.com/wp-content/uploads/2022/01/1642651008435.gif");

    embed
}

pub fn tyuuname() -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    embed
        .title("無礼るなよ")
        .footer(|f| f.text("ﾙﾅﾙﾅ"))
        .image("https://notissary.net/media/2021/06/a.jpg");

    embed
}

pub fn urabetu() -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    embed
        .title("但し")
        .footer(|f| f.text("ｳﾗﾗﾁｬﾝｶﾜｲｲ"))
        .image("https://umamusume.gamerstand.net/wp-content/uploads/2021/08/1622687512216.jpg");

    embed
}

pub fn hello_tenjyo() -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    embed
        .title("Hello Tenjyo")
        .image("https://objectstorage.ap-tokyo-1.oraclecloud.com/n/nr7eduszgfzb/b/image-bucket/o/bot%2Fhello-tenjouunknown.png");

    embed
}
