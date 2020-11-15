use crate::discord;
use crate::structs::{ApplicationData, DiscordGuild};
use actix_web::{web, Responder};
use std::sync::Mutex;

pub async fn discord_user_data(data: web::Data<Mutex<ApplicationData>>) -> impl Responder {
    let auth_cache;
    {
        let data = data.lock().unwrap();
        auth_cache = data.auth_cache.clone();
    }

    let session = discord::get_session(auth_cache).await;

    let discord_user = discord::users_me(format!("{}", session.access_token))
        .await
        .unwrap();

    {
        let mut data = data.lock().unwrap();
        data.auth_cache = Some(session);
    }

    return web::Json(discord_user);
}

pub async fn discord_user_guilds(data: web::Data<Mutex<ApplicationData>>) -> impl Responder {
    let auth_cache;
    let allowed_ids;
    {
        let data = data.lock().unwrap();
        auth_cache = data.auth_cache.clone();
        allowed_ids = data.filter.guild_ids.clone();
    }

    let session = discord::get_session(auth_cache).await;

    let discord_guilds = discord::user_guilds(format!("{}", session.access_token))
        .await
        .unwrap();

    {
        let mut data = data.lock().unwrap();
        data.auth_cache = Some(session);
    }

    // filter guilds to only needed ids to prevent api misuse
    let filtered_guilds: Vec<DiscordGuild> = discord_guilds
        .into_iter()
        .filter(|x| allowed_ids.contains(&x.id))
        .collect();

    return web::Json(filtered_guilds);
}
