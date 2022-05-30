use crate::permissions::Permissions;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: Box<str>,
    pub discriminator: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub guild_id: u64,
    pub roles: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    #[serde(rename = "type")]
    pub kind: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(rename = "type")]
    pub kind: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    pub guild_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: Box<str>,
    pub proxy_url: Box<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    pub kind: u8,
    pub content: Box<str>,
    pub timestamp: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<u64>,
    pub channel_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub guild_id: u64,
    pub position: i64,
    pub permissions: Permissions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub name: Box<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<u8>,
    pub owner_id: u64,
}
