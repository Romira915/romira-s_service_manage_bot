#[derive(Debug, Default)]

pub struct GameServerExecutingState {
    pub minecraft_server_mgpf: bool,
    pub sdtd_server: bool,
    pub terraria_server: bool,
    pub ark_server: bool,
    pub ark_server_second: bool,
    pub ark_server_third: bool,
}

impl GameServerExecutingState {
    pub fn current_executing_count(&self) -> usize {
        self.minecraft_server_mgpf as usize
            + self.sdtd_server as usize
            + self.terraria_server as usize
            + self.ark_server as usize
            + self.ark_server_second as usize
            + self.ark_server_third as usize
    }
}

pub enum Game {
    MinecraftServerMgpf,
    SdtdServer,
    TerrariaServer,
    ArkServer,
    ArkServerSecond,
    ArkServerThird,
}

impl Game {
    pub fn to_service_name(&self) -> String {
        match self {
            Game::MinecraftServerMgpf => "minecraft-server-mgpf.service".to_string(),
            Game::SdtdServer => "sdtd-server.service".to_string(),
            Game::TerrariaServer => "terraria-server.service".to_string(),
            Game::ArkServer => "ark-server.service".to_string(),
            Game::ArkServerSecond => "ark-server-second.service".to_string(),
            Game::ArkServerThird => "ark-server-third.service".to_string(),
        }
    }
}
