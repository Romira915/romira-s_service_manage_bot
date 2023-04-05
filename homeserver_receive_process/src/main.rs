use std::{
    env,
    fs::File,
    io::{self, Read},
    path::PathBuf,
    process::Output,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{bail, Context, Result};
use duct::cmd;
use homeserver_receive_process::{
    home_server_config::Config,
    init_logger,
    models::{Game, GameServerExecutingState},
    Command,
};

const CONFIG_PATH: &str = ".config/home_server_config.toml";

async fn exec_systemctl(command: &web::Json<Command>, service: Game) -> Result<Output> {
    #[allow(clippy::if_same_then_else)]
    if let "start" | "status" = command.request().as_str() {
        // start and status
    } else if let ("stop" | "restart", true) = (command.request().as_str(), command.administrator())
    {
        // stop and restart with admin
    } else {
        bail!("Not Allowed command");
    }

    let request = if command.request() == "status" {
        "is-active"
    } else {
        command.request()
    };

    let result = cmd!("systemctl", request, service.to_service_name())
        .stdout_capture()
        .stderr_capture()
        .run()
        .context("Failed to systemctl");

    result
}

fn into_response_by_cmd_output(output: &Result<Output>) -> HttpResponse {
    match output {
        Ok(output) => {
            let exit_code = output.status;
            log::info!("exit code {}", exit_code);

            if exit_code.success() {
                HttpResponse::Ok().body("Success".to_string())
            } else {
                HttpResponse::ExpectationFailed()
                    .body(format!("Failed to systemctl\n{}", exit_code))
            }
        }
        Err(e) => HttpResponse::ExpectationFailed().body(format!("Failed to cmd! macro\n{}", e)),
    }
}

fn into_response_by_cmd_output_with_status(output: &Result<Output>) -> HttpResponse {
    match output {
        Ok(output) => {
            let exit_code = output.status;
            log::info!("exit code {}", exit_code);

            if exit_code.success() {
                HttpResponse::Ok().body("active".to_string())
            } else {
                HttpResponse::ExpectationFailed().body("inactive".to_string())
            }
        }
        Err(e) => HttpResponse::ExpectationFailed().body("inactive".to_string()),
    }
}

#[post("/minecraft")]
async fn post_minecraft(
    command: web::Json<Command>,
    state: web::Data<Arc<Mutex<GameServerExecutingState>>>,
) -> impl Responder {
    log::info!("post minecraft");

    if state.lock().unwrap().current_executing_count() >= 2 {
        return HttpResponse::ExpectationFailed().body("Two games have already been activated.");
    }

    let result = exec_systemctl(&command, Game::MinecraftServerMgpf).await;

    if let Ok(_) = &result {
        if let "start" | "restart" = command.request().as_str() {
            state.lock().unwrap().minecraft_server_mgpf = true;
        } else if let "stop" = command.request().as_str() {
            state.lock().unwrap().minecraft_server_mgpf = false;
        }
    }

    if command.request() == "status" {
        into_response_by_cmd_output_with_status(&result)
    } else {
        into_response_by_cmd_output(&result)
    }
}

#[post("/sdtd")]
async fn post_sdtd(
    command: web::Json<Command>,
    state: web::Data<Arc<Mutex<GameServerExecutingState>>>,
) -> impl Responder {
    log::info!("post sdtd");

    if state.lock().unwrap().current_executing_count() >= 2 {
        return HttpResponse::ExpectationFailed().body("Two games have already been activated.");
    }

    let result = exec_systemctl(&command, Game::SdtdServer).await;

    if let Ok(_) = &result {
        if let "start" | "restart" = command.request().as_str() {
            state.lock().unwrap().sdtd_server = true;
        } else if let "stop" = command.request().as_str() {
            state.lock().unwrap().sdtd_server = false;
        }
    }

    if command.request() == "status" {
        into_response_by_cmd_output_with_status(&result)
    } else {
        into_response_by_cmd_output(&result)
    }
}

#[post("/terraria")]
async fn post_terraria(
    command: web::Json<Command>,
    state: web::Data<Arc<Mutex<GameServerExecutingState>>>,
) -> impl Responder {
    log::info!("post terraria");

    if state.lock().unwrap().current_executing_count() >= 2 {
        return HttpResponse::ExpectationFailed().body("Two games have already been activated.");
    }

    let result = exec_systemctl(&command, Game::TerrariaServer).await;

    if let Ok(_) = &result {
        if let "start" | "restart" = command.request().as_str() {
            state.lock().unwrap().terraria_server = true;
        } else if let "stop" = command.request().as_str() {
            state.lock().unwrap().terraria_server = false;
        }
    }

    if command.request() == "status" {
        into_response_by_cmd_output_with_status(&result)
    } else {
        into_response_by_cmd_output(&result)
    }
}

#[post("/ark-first")]
async fn post_ark(
    command: web::Json<Command>,
    state: web::Data<Arc<Mutex<GameServerExecutingState>>>,
) -> impl Responder {
    log::info!("post ark");

    if state.lock().unwrap().current_executing_count() >= 2 {
        return HttpResponse::ExpectationFailed().body("Two games have already been activated.");
    }

    let result = exec_systemctl(&command, Game::ArkServer).await;

    if let Ok(_) = &result {
        if let "start" | "restart" = command.request().as_str() {
            state.lock().unwrap().ark_server = true;
        } else if let "stop" = command.request().as_str() {
            state.lock().unwrap().ark_server = false;
        }
    }

    if command.request() == "status" {
        into_response_by_cmd_output_with_status(&result)
    } else {
        into_response_by_cmd_output(&result)
    }
}

#[post("/ark-second")]
async fn post_ark_second(
    command: web::Json<Command>,
    state: web::Data<Arc<Mutex<GameServerExecutingState>>>,
) -> impl Responder {
    log::info!("post ark-second");

    if state.lock().unwrap().current_executing_count() >= 2 {
        return HttpResponse::ExpectationFailed().body("Two games have already been activated.");
    }

    let result = exec_systemctl(&command, Game::ArkServerSecond).await;

    if let Ok(_) = &result {
        if let "start" | "restart" = command.request().as_str() {
            state.lock().unwrap().ark_server_second = true;
        } else if let "stop" = command.request().as_str() {
            state.lock().unwrap().ark_server_second = false;
        }
    }

    if command.request() == "status" {
        into_response_by_cmd_output_with_status(&result)
    } else {
        into_response_by_cmd_output(&result)
    }
}

#[post("/ark-third")]
async fn post_ark_third(
    command: web::Json<Command>,
    state: web::Data<Arc<Mutex<GameServerExecutingState>>>,
) -> impl Responder {
    log::info!("post ark-third");

    if state.lock().unwrap().current_executing_count() >= 2 {
        return HttpResponse::ExpectationFailed().body("Two games have already been activated.");
    }

    let result = exec_systemctl(&command, Game::ArkServerThird).await;

    if let Ok(_) = &result {
        if let "start" | "restart" = command.request().as_str() {
            state.lock().unwrap().ark_server_third = true;
        } else if let "stop" = command.request().as_str() {
            state.lock().unwrap().ark_server_third = false;
        }
    }

    if command.request() == "status" {
        into_response_by_cmd_output_with_status(&result)
    } else {
        into_response_by_cmd_output(&result)
    }
}

#[get("/test")]
async fn index(state: web::Data<Arc<Mutex<GameServerExecutingState>>>) -> impl Responder {
    state.lock().unwrap().ark_server = true;
    state.lock().unwrap().ark_server_second = true;
    HttpResponse::Ok().body(format!("{:?}", state))
}

#[get("/current-executing-count")]
async fn get_current_executing_count(
    state: web::Data<Arc<Mutex<GameServerExecutingState>>>,
) -> impl Responder {
    HttpResponse::Ok().body(state.lock().unwrap().current_executing_count().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let exe_dir = if cfg!(debug_assertions) {
        PathBuf::from_str("./").unwrap()
    } else {
        env::current_exe().unwrap().parent().unwrap().to_path_buf()
    };

    let config: Config = {
        let mut config_full_path = exe_dir.clone();
        config_full_path.push(CONFIG_PATH);
        let mut file = File::open(config_full_path).expect("file not found");

        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str).unwrap();

        toml::from_str(&toml_str).expect("Fall to toml parser")
    };

    HttpServer::new(move || {
        let mut well_known_path = exe_dir.clone();
        well_known_path.push(".well-known");
        App::new()
            .data_factory(|| async {
                Ok::<_, ()>(Arc::new(Mutex::new(GameServerExecutingState::default())))
            })
            .service(index)
            .service(post_minecraft)
            .service(post_sdtd)
            .service(post_terraria)
            .service(post_ark)
            .service(post_ark_second)
            .service(post_ark_third)
            .service(get_current_executing_count)
            .service(Files::new("/.well-known", well_known_path.as_path()))
    })
    .client_request_timeout(Duration::from_millis(30000))
    .client_disconnect_timeout(Duration::from_millis(30000))
    .bind(format!(
        "{}:{}",
        config.address().home_server_bind_ip(),
        config.address().home_server_bind_port()
    ))?
    .run()
    .await
}
