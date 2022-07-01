use std::{env, fs::File, io::Read};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use duct::cmd;
use homeserver_receive_process::{home_server_config::Config, Command};

const CONFIG_PATH: &'static str = ".config/home_server_config.toml";

async fn exec_systemctl(command: web::Json<Command>, service_name: &str) -> impl Responder {
    #[allow(clippy::if_same_then_else)]
    if let "start" | "status" = command.request().as_str() {
        // start and status
    } else if let ("stop" | "restart", true) =
        (&*command.request().as_str(), command.administrator())
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
                    .body(format!("Failed to systemctl\n{}", exit_code.to_string()))
            }
        }
        Err(e) => {
            if command.request() == "status" {
                HttpResponse::ExpectationFailed().body("inactive (dead)")
            } else {
                HttpResponse::ExpectationFailed()
                    .body(format!("Failed to cmd! macro\n{}", e.to_string()))
            }
        }
    };

    response
}

#[post("/minecraft")]
async fn post_minecraft(command: web::Json<Command>) -> impl Responder {
    exec_systemctl(command, "minecraft-server-mgpf.service").await
}

#[post("/sdtd")]
async fn post_sdtd(command: web::Json<Command>) -> impl Responder {
    exec_systemctl(command, "sdtd-server.service").await
}

#[post("/terraria")]
async fn post_terraria(command: web::Json<Command>) -> impl Responder {
    exec_systemctl(command, "terraria-server.service").await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = {
        let mut exe_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        exe_dir.push(CONFIG_PATH);
        let mut file = File::open(exe_dir).expect("file not found");

        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str).unwrap();

        toml::from_str(&toml_str).expect("Fall to toml parser")
    };

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(post_minecraft)
            .service(post_sdtd)
            .service(post_terraria)
    })
    .bind(format!(
        "{}:{}",
        config.address().home_server_bind_ip(),
        config.address().home_server_bind_port()
    ))?
    .run()
    .await
}
