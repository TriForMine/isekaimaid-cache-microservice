use serde_derive::{Deserialize, Serialize};

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
