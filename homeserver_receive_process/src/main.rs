use std::iter::Sum;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use duct::cmd;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    service: Option<String>,
    request: String,
    subrequest: Option<String>,
    administrator: bool,
}

#[post("/minecraft")]
async fn post_minecraft(command: web::Json<Command>) -> impl Responder {
    if let "start" | "status" = &*command.request {
    } else if let ("stop" | "restart", true) = (&*command.request, command.administrator) {
    } else {
        return HttpResponse::MethodNotAllowed().body("Not Allowed command");
    }

    let response = match cmd!(
        "systemctl",
        &command.request,
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
    println!("start HttpServer");
    HttpServer::new(|| App::new().service(post_minecraft))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
