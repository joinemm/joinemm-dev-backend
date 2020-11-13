use crate::discord;
use actix_web::{web, Responder};

pub async fn discord_user_data() -> impl Responder {
    let session = discord::authenticate().await.unwrap();

    let discord_user = discord::users_me(format!("{}", session.access_token))
        .await
        .unwrap();

    return web::Json(discord_user);
}
