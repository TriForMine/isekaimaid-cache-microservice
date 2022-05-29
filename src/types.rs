use dashmap::DashMap;
use serde_derive::{Deserialize, Serialize};
use crate::permissions::Permissions;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub discriminator: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: u64,
    pub guild_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub  roles: Option<Vec<u64>>,
    pub cached_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(rename = "type")]
    pub kind: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,
    pub guild_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscordEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub name: String,
    pub animated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub proxy_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    pub kind: u8,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<DashMap<u64, Role>>,
    pub id: u64,
    pub owner_id: u64,
}
