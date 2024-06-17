use serde::{Deserialize, Serialize};

use super::core_types::*;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(untagged)]
// pub enum ExtendsActor {
//     Actor(ActorWrapper),
//     Application(ApplicationWrapper),
//     Group(GroupWrapper),
//     Organization(OrganizationWrapper),
//     Person(PersonWrapper),
//     Service(ServiceWrapper),
// }

// impl IsActor for ExtendsActor {
//     fn actor<'a>(&'a self) -> &'a Actor {
//         let val: &'a Actor = match self {
//             ExtendsActor::Actor(ActorWrapper::Actor(x)) => {
//                 x
//             },
//             ExtendsActor::Application(ApplicationWrapper::Application(x)) => {
//                 x.actor()
//             },
//             ExtendsActor::Group(x) => {
//                 let val = match x {
//                     GroupWrapper::Group(x) => x,
//                 };
//                 val.actor()
//             },
//             ExtendsActor::Organization(x) => {
//                 let val = match x {
//                     OrganizationWrapper::Organization(x) => x,
//                 };
//                 val.actor()
//             },
//             ExtendsActor::Person(x) => {
//                 let val = match x {
//                     PersonWrapper::Person(x) => x,
//                 };
//                 val.actor()
//             },
//             ExtendsActor::Service(x) => {
//                 let val = match x {
//                     ServiceWrapper::Service(x) => x,
//                 };
//                 val.actor()
//             },
//         };

//         &val
//     }
//     fn actor_type(&self) -> ActorType {
//         match self {
//             ExtendsActor::Actor(_) => ActorType::Actor,
//             ExtendsActor::Application(_) => ActorType::Application,
//             ExtendsActor::Group(_) => ActorType::Group,
//             ExtendsActor::Organization(_) => ActorType::Organization,
//             ExtendsActor::Person(_) => ActorType::Person,
//             ExtendsActor::Service(_) => ActorType::Service,
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorType {
    Actor,
    Application,
    Group,
    Organization,
    Person,
    Service,
}

pub trait IsActor {
    fn actor<'a>(&'a self) -> &'a Actor;
    fn actor_type(&self) -> ActorType;
}

//-------------------types--------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
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

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(tag = "type")]
// pub enum ActorWrapper {
//     Actor(Actor),
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// summary, id, and name are inherited from [`Object`]
pub struct Actor {
    #[serde(rename = "type")]
    pub type_field: ActorType,
    #[serde(flatten)]
    pub preferred_username: String,
    pub extends_object: Object,
    pub public_key: PublicKey,

    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,

    #[serde(skip)]
    pub ap_user_id: Option<i64>,
    #[serde(skip)]
    pub domain: Option<String>,
    #[serde(skip)]
    pub liked: Option<String>,
}

impl Actor {
    pub fn to_activitystream(self) -> ActivityStream {
        ActivityStream {
            content: ContextWrap {
                context: Context::Array(vec![
                    "https://www.w3.org/ns/activitystreams".to_owned(),
                    "https://w3id.org/security/v1".to_owned(),
                ]),
                activity_stream: RangeLinkObject::Object(ExtendsObject::Actor(Box::new(self))),
            },
        }
    }
}

impl From<Actor> for ActivityStream {
    fn from(value: Actor) -> ActivityStream {
        value.to_activitystream()
    }
}

impl From<Box<Actor>> for ActivityStream {
    fn from(value: Box<Actor>) -> ActivityStream {
        ActivityStream {
            content: ContextWrap {
                context: Context::Array(vec![
                    "https://www.w3.org/ns/activitystreams".to_owned(),
                    "https://w3id.org/security/v1".to_owned(),
                ]),
                activity_stream: RangeLinkObject::Object(ExtendsObject::Actor(value)),
            },
        }
    }
}

impl IsActor for &Actor {
    fn actor<'a>(&'a self) -> &'a Actor {
        &self
    }
    fn actor_type(&self) -> ActorType {
        ActorType::Actor
    }
}
