use discord_bot_client::commands::minecraft::MinecraftCommand;

#[test]
fn enum_to_string() {
    assert_eq!("start".to_string(), MinecraftCommand::Start.to_string());
    assert_eq!("status".to_string(), MinecraftCommand::Status.to_string());
    assert_eq!("stop".to_string(), MinecraftCommand::Stop.to_string());
    assert_eq!("restart".to_string(), MinecraftCommand::Restart.to_string());
}
