use discord_bot_client::commands::minecraft::SystemctlCommand;

#[test]
fn enum_to_string() {
    assert_eq!("start".to_string(), SystemctlCommand::Start.to_string());
    assert_eq!("status".to_string(), SystemctlCommand::Status.to_string());
    assert_eq!("stop".to_string(), SystemctlCommand::Stop.to_string());
    assert_eq!("restart".to_string(), SystemctlCommand::Restart.to_string());
}
