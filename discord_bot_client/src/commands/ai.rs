use std::ffi::{CStr, CString};
use std::time::Duration;

use duct::cmd;
use nix::unistd::{self, ForkResult};
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::Context;
use serenity::prelude::*;

#[group]
#[commands(draw)]
#[prefixes("ai", "ml")]
#[description = "機械学習コマンド"]
pub struct Ai;

#[command]
pub async fn draw(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    log::info!("{:?}", args.message());

    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            log::debug!("Main({}) forked a child({})", unistd::getpid(), child);
        }
        Ok(ForkResult::Child) => {
            log::debug!(
                "Child({}) started. PPID is {}",
                unistd::getpid(),
                unistd::getppid()
            );
            unistd::execv(
                CString::new("/home/romira/miniconda3/envs/browser/bin/python")
                    .unwrap()
                    .as_c_str(),
                &[
                    CString::new("/home/romira/miniconda3/envs/browser/bin/python")
                        .unwrap()
                        .as_c_str(),
                    CString::new("./colab-cli/main.py").unwrap().as_c_str(),
                ],
            )
            .unwrap();
        }
        Err(_) => log::debug!("Fork failed"),
    }

    msg.channel_id
        .send_message(&ctx.http, |m| m.content("絵を書くよ～(o・∇・o)"))
        .await?;

    Ok(())
}
