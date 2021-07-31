pub mod home_server_config;
use std::default;

use derive_builder::Builder;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Builder, Getters)]
pub struct Command {
    #[getset(get = "pub")]
    #[builder(default)]
    service: Option<String>,
    #[getset(get = "pub")]
    request: String,
    #[getset(get = "pub")]
    #[builder(default)]
    subrequest: Option<String>,
    #[getset(get = "pub")]
    administrator: bool,
}
