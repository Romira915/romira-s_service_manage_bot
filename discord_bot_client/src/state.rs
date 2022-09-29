use std::ops::DerefMut;

use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

#[derive(
    Debug, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Default, Clone,
)]
pub struct BotState {
    pub is_quiet: bool,
}

pub struct BotStateContainer;

impl TypeMapKey for BotStateContainer {
    type Value = BotState;
}
