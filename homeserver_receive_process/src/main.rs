use std::{env, fs::File, io::Read, time::Duration};

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use duct::cmd;
use homeserver_receive_process::{home_server_config::Config, init_logger, Command};

const CONFIG_PATH: &str = ".config/home_server_config.toml";

async fn exec_systemctl(command: web::Json<Command>, service_name: &str) -> impl Responder {
    #[allow(clippy::if_same_then_else)]
    if let "start" | "status" = command.request().as_str() {
        // start and status
    } else if let ("stop" | "restart", true) = (command.request().as_str(), command.administrator())
    {
        // stop and restart with admin
    } else {
        return HttpResponse::MethodNotAllowed().body("Not Allowed command");
    }

    let response = match cmd!("systemctl", &command.request(), service_name)
        .stdout_capture()
        .stderr_capture()
        .run()
    {
        Ok(output) => {
            let exit_code = output.status;
            log::info!("exit code {}", exit_code);

            if exit_code.success() {
                let content = if command.request() == "status" {
                    let output = String::from_utf8(output.stdout).unwrap();
                    let mut split = output.split_whitespace();
                    split.position(|p| p == "Active:");

                    format!("{} {}", split.next().unwrap(), split.next().unwrap())
                } else {
                    "Success".to_string()
                };

                HttpResponse::Ok().body(content)
            } else {
                HttpResponse::ExpectationFailed()
                    .body(format!("Failed to systemctl\n{}", exit_code))
            }
        }
        Err(e) => {
            if command.request() == "status" {
                HttpResponse::ExpectationFailed().body("inactive (dead)")
            } else {
                HttpResponse::ExpectationFailed().body(format!("Failed to cmd! macro\n{}", e))
            }
        }
    };

    response
}

#[post("/minecraft")]
async fn post_minecraft(command: web::Json<Command>) -> impl Responder {
    log::info!("post minecraft");
    exec_systemctl(command, "minecraft-server-mgpf.service").await
}

#[post("/sdtd")]
async fn post_sdtd(command: web::Json<Command>) -> impl Responder {
    log::info!("post sdtd");
    exec_systemctl(command, "sdtd-server.service").await
}

#[post("/terraria")]
async fn post_terraria(command: web::Json<Command>) -> impl Responder {
    log::info!("post terraria");
    exec_systemctl(command, "terraria-server.service").await
}

#[post("/ark")]
async fn post_ark(command: web::Json<Command>) -> impl Responder {
    log::info!("post ark");
    exec_systemctl(command, "ark-server.service").await
}

#[get("/test")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let exe_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();

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
            .service(index)
            .service(post_minecraft)
            .service(post_sdtd)
            .service(post_terraria)
            .service(post_ark)
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
