#[macro_use]
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct Config {
    #[getset(get = "pub")]
    address: Address,
    #[getset(get = "pub")]
    discord: Discord,
    #[getset(get = "pub")]
    gameinfo: Option<GameInfo>,
    #[getset(get = "pub")]
    log: Log,
    #[getset(get = "pub")]
    secret: Secret,
    #[getset(get = "pub")]
    path: Path,
}

pub struct ConfigContainer;

impl TypeMapKey for ConfigContainer {
    type Value = Config;
}

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct Address {
    #[getset(get = "pub")]
    home_server_ip: String,
    #[getset(get = "pub")]
    home_server_macaddress: String,
    #[getset(get_copy = "pub")]
    home_server_port: u32,
    #[getset(get_copy = "pub")]
    home_server_wol_port: u32,
    #[getset(get_copy = "pub")]
    valheim_confirmation_port: u32,
}

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct Discord {
    #[getset(get = "pub")]
    administrator: Option<Vec<u64>>,
    #[getset(get = "pub")]
    token: String,
    #[getset(get = "pub")]
    prefix: String,
}

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct GameInfo {
    #[getset(get = "pub")]
    sdtd_password: Option<String>,
}

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct Log {
    #[getset(get = "pub")]
    term_log: String,
    #[getset(get = "pub")]
    write_log: String,
}

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct Secret {
    #[getset(get = "pub")]
    rinna_subscription_key: String,
    #[getset(get = "pub")]
    rinna_cce_subscription_key: String,
}

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct Path {
    #[getset(get = "pub")]
    python: String,
    #[getset(get = "pub")]
    geckodriver_path: String,
    #[getset(get = "pub")]
    firefox_profile: String,
}
