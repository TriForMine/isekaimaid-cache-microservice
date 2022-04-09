use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u64,
    username: String,
    discriminator: u8,
    avatar: String,
    bot: bool,
    locale: Option<String>
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
    name: String,
    guild_id: u64,
    permission_overwrites: Vec<String>,
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    id: u64,
    filename: String,
    content_type: Option<String>,
    size: u16,
    url: String,
    proxy_url: String,
    height: Option<u32>,
    width: Option<u32>,
    ephemeral: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscordEmoji {
    id: Option<u64>,
    name: String,
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
    url: String,
    proxy_url: String,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmbedAuthor {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmbedField {
    name: Option<String>,
    value: String,
    inline: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmbedFooter {
    text: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    kind: String,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<u32>,
    footer: Option<EmbedFooter>,
    image: Option<Image>,
    thumbnail: Option<Image>,
    video: Option<Image>,
    provider: Option<String>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageInteraction {
    id: u64,
    #[serde(rename = "type")]
    kind: Option<u8>,
    name: String,
    user: User
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectOption {
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<DiscordEmoji>,
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
    #[serde(rename = "type")]
    kind: Option<u8>,
    custom_id: Option<String>,
    disabled: Option<bool>,
    style: Option<ComponentStyle>,
    label: Option<String>,
    value: Option<String>,
    emoji: DiscordEmoji,
    url: Option<String>,
    options: Option<Vec<SelectOption>>,
    placeholder: Option<String>,
    min_values: Option<u8>,
    max_values: Option<u8>,
    components: Option<Vec<Component>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    kind: u8,
    content: String,
    timestamp: Option<String>,
    tag: String,
    edited_timestamp: Option<String>,
    attachments: Option<Vec<Attachment>>,
    embeds: Option<Vec<Embed>>,
    reactions: Option<Vec<MessageReaction>>,
    interaction: Option<MessageInteraction>,
    components: Option<Vec<Component>>,
    id: u64,
    guild_id: Option<u64>,
    author_id: Option<u64>,
    user: Option<User>,
    member: Option<Member>
}
