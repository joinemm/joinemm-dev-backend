use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DiscordAuthentication {
    pub token_type: String,
    pub access_token: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DiscordUserData {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DiscordGuild {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub owner: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DiscordGuildsList {
    pub guilds: Vec<DiscordGuild>,
}

#[derive(Deserialize, Debug)]
pub struct Filter {
    pub allowed_origin: String,
    pub guild_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationData {
    pub filter: Filter,
}
