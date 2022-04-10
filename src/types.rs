use dashmap::DashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u64,
    username: Box<String>,
    discriminator: u8,
    avatar: Box<String>,
    bot: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<Box<String>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    id: u64,
    guild_id: u64,
    roles: Vec<u64>,
    cached_at: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(rename = "type")]
    kind: u8,
    name: Box<String>,
    guild_id: u64,
    permission_overwrites: Vec<Box<String>>,
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    id: u64,
    filename: Box<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<Box<String>>,
    size: u16,
    url: Box<String>,
    proxy_url: Box<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscordEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    name: Box<String>,
    animated: bool
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageReaction {
    me: bool,
    count: u32,
    emoji: DiscordEmoji
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    url: Box<String>,
    proxy_url: Box<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmbedAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_icon_url: Option<Box<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmbedField {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Box<String>>,
    value: Box<String>,
    inline: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmbedFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_icon_url: Option<Box<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    title: Option<Box<String>>,
    #[serde(rename = "type")]
    kind: Box<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<EmbedAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<EmbedField>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageInteraction {
    id: u64,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    kind: Option<u8>,
    name: Box<String>,
    user: User
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectOption {
    label: Box<String>,
    value: Box<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<DiscordEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary,
    Success,
    Danger,
    Link,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TextStyle {
    Short = 1,
    Paragraph = 2,
}

#[derive(Serialize, Deserialize, Debug)]
enum ComponentStyle {
    ButtonStyle(ButtonStyle),
    TextStyle(TextStyle)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    kind: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_id: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ComponentStyle>,
    label: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    emoji: DiscordEmoji,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<SelectOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_values: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_values: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<Component>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    kind: u8,
    content: Box<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<Box<String>>,
    tag: Box<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edited_timestamp: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reactions: Option<Vec<MessageReaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interaction: Option<MessageInteraction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<Component>>,
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<Member>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unicode_emoji: Option<Box<String>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locale: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    joined_at: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_id: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<DashMap<u64, Role>>,
    id: u64,
    owner_id: u64
}
