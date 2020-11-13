mod discord;
mod handlers;
mod structs;

use actix_web::{web, App, HttpServer};
use kankyo;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    // Start http server
    HttpServer::new(move || {
        App::new()
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
