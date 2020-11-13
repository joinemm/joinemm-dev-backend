use crate::discord;
use crate::structs::{ApplicationData, DiscordGuild};
use actix_web::{web, Responder};
use std::sync::Mutex;

pub async fn discord_user_data() -> impl Responder {
    let session = discord::authenticate().await.unwrap();

    let discord_user = discord::users_me(format!("{}", session.access_token))
        .await
        .unwrap();

    return web::Json(discord_user);
}

pub async fn discord_user_guilds(data: web::Data<Mutex<ApplicationData>>) -> impl Responder {
    let data = data.lock().unwrap();
    let session = discord::authenticate().await.unwrap();

    let discord_guilds = discord::user_guilds(format!("{}", session.access_token))
        .await
        .unwrap();

    // filter guilds to only needed ids to prevent api misuse
    let allowed_ids = &data.filter.guild_ids;
    let filtered_guilds: Vec<DiscordGuild> = discord_guilds
        .into_iter()
        .filter(|x| allowed_ids.contains(&x.id))
        .collect();

    return web::Json(filtered_guilds);
}
