use std::ops::DerefMut;

use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

#[derive(Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Clone)]
pub struct BotState {
    pub is_quiet: bool,
}

impl Default for BotState {
    fn default() -> Self {
        BotState { is_quiet: true }
    }
}

pub struct BotStateContainer;

impl TypeMapKey for BotStateContainer {
    type Value = BotState;
}
