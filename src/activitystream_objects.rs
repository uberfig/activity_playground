use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ActorType {
    Person,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "type")]
    pub type_field: ActorType,
    pub id: String,
    pub name: String,
    pub preferred_username: String,
    pub summary: String,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub liked: String,
}
#[derive(Serialize, Deserialize)]
pub enum ObjectType {
    Note,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamObject {
    #[serde(rename = "type")]
    pub type_field: ObjectType,
    pub id: String,
    pub attributed_to: String,
    pub to: Vec<String>,
    pub in_reply_to: Option<String>,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub enum ActivityType {
    Create,
    Like,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityObjType {
    Object(StreamObject),
    Link(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "type")]
    pub type_field: ActivityType,
    pub id: String,
    pub to: Vec<String>,
    pub actor: String,
    pub object: ActivityObjType,
}
