#[macro_use]
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default)]
pub struct Config {
    #[getset(get = "pub")]
    address: Address,
}

#[derive(Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default)]
pub struct Address {
    #[getset(get = "pub")]
    home_server_bind_ip: String,
    #[getset(get_copy = "pub")]
    home_server_bind_port: u32,
}
