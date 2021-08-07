use serenity::builder::CreateEmbed;

pub fn www() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://pbs.twimg.com/profile_images/879510459027562496/jA8ScZnS_400x400.jpg");

    embed
}

pub fn yosi() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
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
    embed.image("https://i.pinimg.com/564x/d3/75/8d/d3758dd7baa347b799d457c5306cd294.jpg");

    embed
}

pub fn ai_chan() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.image("https://matome.hacker-hacker.com/wp-content/uploads/2020/05/genba-1.gif");

    embed
}