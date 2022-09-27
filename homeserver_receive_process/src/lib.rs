pub mod home_server_config;

use derive_builder::Builder;
use getset::Getters;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

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

pub fn init_logger() {
    CombinedLogger::init(vec![
        #[cfg(not(feature = "termcolor"))]
        TermLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_time_offset_to_local()
                .unwrap()
                .build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])
    .unwrap()
}
