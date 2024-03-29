extern crate discord_bot_client;

use discord_bot_client::{
    bot_config::{Config as BotConfig, ConfigContainer},
    commands::{
        activity::ACTIVITYCOMMAND_GROUP, ai::*, ark::ARK_GROUP, minecraft::*, sdtd::*, terraria::*,
        valheim::*,
    },
    state::{BotState, BotStateContainer},
    *,
};
use log::error;
use serenity::{
    client::bridge::gateway::ShardManager, framework::StandardFramework, http::Http, prelude::*,
};
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
    WriteLogger,
};
use std::{collections::HashSet, path::PathBuf, sync::Arc};
use std::{env, fs::File};
use std::{io::prelude::*, str::FromStr};

const CONFIG_PATH: &str = ".config/bot_config.toml";

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

fn init_logger(config: &BotConfig) {
    CombinedLogger::init(vec![
        #[cfg(not(feature = "termcolor"))]
        TermLogger::new(
            LevelFilter::from_str(config.log().term_log()).unwrap(),
            ConfigBuilder::new().set_time_to_local(true).build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::from_str(config.log().write_log()).unwrap(),
            Config::default(),
            File::create("bot.log").unwrap(),
        ),
    ])
    .unwrap()
}

#[tokio::main]
async fn main() {
    // dotenv::dotenv().expect("Failed to load .env file");
    // if let Err(_) = env::var("RUST_LOG") {
    //     env::set_var("RUST_LOG", "info");
    // }

    let config: BotConfig = {
        let mut exe_dir = if cfg!(debug_assertions) {
            PathBuf::new()
        } else {
            env::current_exe().unwrap().parent().unwrap().to_path_buf()
        };
        exe_dir.push(CONFIG_PATH);
        let mut file = File::open(exe_dir).expect("file not found");

        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str).unwrap();

        toml::from_str(&toml_str).expect("Fall to toml parser.")
    };

    let bot_state = BotState::default();

    init_logger(&config);

    let token = config.discord().token();

    let http = Http::new(token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(config.discord().prefix()))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MINECRAFT_GROUP)
        .group(&VALHEIM_GROUP)
        .group(&SDTD_GROUP)
        .group(&TERRARIA_GROUP)
        .group(&ARK_GROUP)
        .group(&ACTIVITYCOMMAND_GROUP)
        .group(&AI_GROUP);
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creationg client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<ConfigContainer>(config);
        data.insert::<BotStateContainer>(bot_state);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
