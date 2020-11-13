use crate::structs::*;
use reqwest;
use reqwest::header::HeaderMap;
use serde_json;
use std::env;

pub async fn authenticate() -> Result<DiscordAuthentication, reqwest::Error> {
    let client = reqwest::Client::new();
    let client_id = env::var("DISCORD_CLIENT_ID").unwrap();
    let client_secret = env::var("DISCORD_CLIENT_SECRET").unwrap();
    let param = reqwest::multipart::Form::new()
        .text("grant_type", "client_credentials")
        .text("scope", "identify guilds");

    let response = client
        .post("https://discord.com/api/oauth2/token")
        //.headers(headers)
        .multipart(param)
        .basic_auth(client_id, Some(client_secret))
        .send()
        .await?;

    Ok(response.json::<DiscordAuthentication>().await?)
}

pub async fn users_me(token: String) -> Result<DiscordUserData, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let response = client
        .get("https://discord.com/api/v6/users/@me")
        .headers(headers)
        .send()
        .await?;

    let text = response.text().await?;
    let content = match serde_json::from_str::<DiscordUserData>(&text) {
        Result::Ok(val) => val,
        Result::Err(_err) => panic!(format!("Unable to get item: {}", text)),
    };

    Ok(content)
}

pub async fn user_guilds(token: String) -> Result<Vec<DiscordGuild>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let response = client
        .get("https://discord.com/api/v6/users/@me/guilds")
        .headers(headers)
        .send()
        .await?;

    let text = response.text().await?;
    let content = match serde_json::from_str::<Vec<DiscordGuild>>(&text) {
        Result::Ok(val) => val,
        Result::Err(_err) => panic!(format!("Unable to get item: {}", text)),
    };

    Ok(content)
}
