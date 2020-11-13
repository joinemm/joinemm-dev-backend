mod discord;
mod handlers;
mod structs;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use kankyo;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    // Start http server
    HttpServer::new( || {
        let cors = Cors::default() // <- Construct CORS middleware builder
            .allowed_origin("http://192.168.1.80:5500")
            .allowed_origin("https://joinemm.dev")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            //.app_data(data.clone())
            .route("/discorduser", web::get().to(handlers::discord_user_data))
            .external_resource(
                "discord_auth", 
                "https://discord.com/api/oauth2/authorize?response_type=code&scope=identify&prompt=consent&redirect_uri={redirect}&client_id={client_id}&state={state}"
            )
    })
    .bind(env::var("BIND").unwrap())?
    .run()
    .await
}
