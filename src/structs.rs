use chrono::NaiveDateTime;
use chrono::Utc;
use serde::de;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone)]
pub struct DiscordAuthentication {
    pub token_type: String,
    pub access_token: String,
    #[serde(deserialize_with = "future_date")]
    pub expires_in: NaiveDateTime,
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

#[derive(Deserialize, Debug, Clone)]
pub struct Filter {
    pub allowed_origin: String,
    pub guild_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationData {
    pub filter: Filter,
    pub auth_cache: Option<DiscordAuthentication>,
}

struct NaiveDateTimeVisitor;

impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "n seconds in the future")
    }

    fn visit_u64<E>(self, n: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let future_timestamp = Utc::now().timestamp() + n as i64;
        Ok(NaiveDateTime::from_timestamp(future_timestamp, 0))
    }
}

/// get a NaiveTimeStamp given seconds in the future, aka the expiration date
fn future_date<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_u64(NaiveDateTimeVisitor)
}
