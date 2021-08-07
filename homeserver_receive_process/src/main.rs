use std::{fs::File, io::Read};

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use duct::cmd;
use homeserver_receive_process::{home_server_config::Config, Command};

const CONFIG_PATH: &'static str = "./.config/home_server_config.toml";

#[post("/minecraft")]
async fn post_minecraft(command: web::Json<Command>) -> impl Responder {
    if let "start" | "status" = command.request().as_str() {
    } else if let ("stop" | "restart", true) =
        (&*command.request().as_str(), command.administrator())
    {
    } else {
        return HttpResponse::MethodNotAllowed().body("Not Allowed command");
    }

    let response = match cmd!(
        "systemctl",
        &command.request(),
        "minecraft-server-mgpf.service"
    )
    .run()
    {
        Ok(output) => {
            let exit_code = output.status;
            if exit_code.success() {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::ExpectationFailed()
                    .body(format!("Failed to systemctl\n{}", exit_code.to_string()))
            }
        }
        Err(e) => HttpResponse::ExpectationFailed()
            .body(format!("Failed to cmd! macro\n{}", e.to_string())),
    };

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = {
        let mut file = File::open(CONFIG_PATH).expect("file not found");

        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str);

        toml::from_str(&toml_str).expect("Fall to toml parser")
    };

    HttpServer::new(|| App::new().service(post_minecraft))
        .bind(format!(
            "{}:{}",
            config.address().home_server_bind_ip(),
            config.address().home_server_bind_port()
        ))?
        .run()
        .await
}
