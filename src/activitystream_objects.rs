use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    pub id: String,    //https://my-example.com/actor#main-key
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
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: ActorType,
    #[serde(skip)]
    pub name: Option<String>,
    pub preferred_username: String,
    #[serde(skip)]
    pub domain: String,
    #[serde(skip)]
    pub summary: String,
    pub inbox: String,
    #[serde(skip)]
    pub outbox: String,
    #[serde(skip)]
    pub followers: String,
    #[serde(skip)]
    pub following: String,
    #[serde(skip)]
    pub liked: Option<String>,

    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
///Actor type for just deserializing the useful bits for verifying post came from an actor
pub struct VerificationActor {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: ActorType,
    pub preferred_username: String,
    pub public_key: PublicKey,
}

impl From<DatabaseActor> for Actor {
    fn from(value: DatabaseActor) -> Self {
        Actor {
            database_id: value.database_id,
            context: vec![
                "https://www.w3.org/ns/activitystreams".to_string(),
                "https://w3id.org/security/v1".to_string(),
            ],
            type_field: value.type_field,
            id: value.id,
            name: value.name,
            preferred_username: value.preferred_username,
            domain: value.domain,
            summary: value.summary,
            inbox: value.inbox,
            outbox: value.outbox,
            followers: value.followers,
            following: value.following,
            liked: value.liked,
            public_key: value.public_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseActor {
    #[serde(skip)]
    pub database_id: i64,
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
    #[serde(skip)]
    pub liked: Option<String>,

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
    pub to: String,
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
pub struct OldActivity {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "type")]
    pub type_field: ActivityType,
    pub id: String,
    // pub to: Vec<String>,
    pub actor: String,
    pub object: StreamObject,
}

//--------------------new implimentaition---------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub id: String,

    pub name: Option<String>,
    //TODO
    pub attachment: Option<String>,
    pub attributed_to: Option<String>,
    pub audience: Option<String>,
    pub content: Option<String>,
    pub end_time: Option<String>,
    pub generator: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub in_reply_to: Option<String>,
    pub location: Option<String>,
    pub preview: Option<String>,
    pub published: Option<String>,
    pub replies: Option<String>,
    pub start_time: Option<String>,
    pub summary: Option<String>,
    pub tag: Option<String>,
    pub updated: Option<String>,
    pub url: Option<String>,
    pub to: Option<String>,
    pub bto: Option<String>,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub media_type: Option<String>,
    pub duration: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    pub hreflang: Option<String>,
    pub media_type: String,
    pub name: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub preview: Option<String>,//TODO
    pub rel: Option<String>//TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    pub extends_object: Object,
    pub actor: Option<String>, //TODO
    pub target: Option<String>, //TODO
    pub result: Option<String>, //TODO
    pub origin: Option<String>, //TODO
    pub instrument: Option<String>, //TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(flatten)]
    pub intransitive: IntransitiveActivity,
    pub object: Option<Box<Object>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(flatten)]
    pub extends_object: Object,
    pub total_items: u32,
    pub current: Option<String>, //TODO
    pub first: Option<String>, //TODO
    pub last: Option<String>, //TODO
    pub items: Option<String>, //TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(flatten)]
    pub extends_collection: Collection,
    pub part_of: Option<String>, //TODO
    pub next: Option<String>, //TODO
    pub prev: Option<String>, //TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(flatten)]
    pub extends_collection: Collection,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Context {
    #[serde(rename = "@context")]
    Array(Vec<String>),
    #[serde(rename = "@context")]
    Single(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ActivityStream {
    Object(Object),
    Link(Link),
    Activity(Activity),
    IntransitiveActivity(IntransitiveActivity),
    Collection(Collection),
    OrderedCollection(OrderedCollection),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextWrap {
    #[serde(flatten)]
    pub context: Context,
    #[serde(flatten)]
    pub activity_stream: ActivityStream,
}
