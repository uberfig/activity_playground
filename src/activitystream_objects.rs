use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ActorType {
    Person,
    Other,
}

impl From<String> for ActorType {
    fn from(value: String) -> Self {
        if value.eq_ignore_ascii_case("Person") {
            return ActorType::Person;
        }
        return ActorType::Other;
    }
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String, //https://my-example.com/actor#main-key
    pub owner: String, //"https://my-example.com/actor"
    pub public_key_pem: String,
}

impl From<String> for PublicKey {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

// impl TryFrom<Option<String>> for PublicKey {
//     type Error = &'static str;
//     fn try_from(value: Option<String>) -> Result<PublicKey, Self::Error> {
//         todo!()
//     }
// }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    #[serde(skip)]
    pub database_id: i64,
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "type")]
    pub type_field: ActorType,
    pub id: String,
    pub name: Option<String>,
    pub preferred_username: String,
    #[serde(skip)]
    pub domain: String,
    pub summary: String,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub liked: String,

    pub public_key: PublicKey,
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
