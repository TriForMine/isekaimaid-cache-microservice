use dashmap::DashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    discriminator: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    id: u64,
    pub(crate) guild_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<u64>>,
    cached_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(rename = "type")]
    kind: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    guild_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<String>>,
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscordEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    name: String,
    animated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageReaction {
    me: bool,
    count: u32,
    emoji: DiscordEmoji,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    url: String,
    proxy_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    kind: u8,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timestamp: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) edited_timestamp: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<Embed>>,
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    joined_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_id: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<DashMap<u64, Role>>,
    id: u64,
    owner_id: u64,
}
