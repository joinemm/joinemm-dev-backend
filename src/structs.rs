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
