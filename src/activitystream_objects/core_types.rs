use serde::{Deserialize, Serialize};

use super::activity_types::*;

//-------------------glue--------------

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
    ExtendsObject(ExtendsObject),
    LinkType(LinkType),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextWrap {
    #[serde(flatten)]
    pub context: Context,
    #[serde(flatten)]
    pub activity_stream: ActivityStream,
}

//--------------primitive-----------------

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
    pub preview: Option<String>, //TODO
    pub rel: Option<String>,     //TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LinkType {
    Simple(String),
    Expanded(Link),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// represents a field that could be an object or a link
pub enum RangeLinkObject {
    Object(ExtendsObject),
    Link(LinkType),
}

//---------------Activities--------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    pub extends_object: Object,
    pub actor: Option<String>,      //TODO
    pub target: Option<String>,     //TODO
    pub result: Option<String>,     //TODO
    pub origin: Option<String>,     //TODO
    pub instrument: Option<String>, //TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(flatten)]
    pub extends_intransitive: IntransitiveActivity,
    pub object: Option<Box<Object>>,
}

// --------------collections----------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(flatten)]
    pub extends_object: Object,
    pub total_items: u32,
    pub current: Option<String>, //TODO
    pub first: Option<String>,   //TODO
    pub last: Option<String>,    //TODO
    pub items: Option<String>,   //TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(flatten)]
    pub extends_collection: Collection,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(flatten)]
    pub extends_collection: Collection,
    pub part_of: Option<String>, //TODO
    pub next: Option<String>,    //TODO
    pub prev: Option<String>,    //TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub extends_collection: Collection,
    #[serde(flatten)]
    pub extends_collection_page: CollectionPage,
}

//--------------------inheritence---------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsObject {
    Object(Object),
    ExtendsIntransitive(ExtendsIntransitive),
    ExtendsCollection(ExtendsCollection),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsActivity {
    Activity(Activity),
    ExtendsAccept(ExtendsAccept),
    Add(Add),
    Create(Create),
    Delete(Delete),
    Follow(Follow),
    ExtendsIgnore(ExtendsIgnore),
    Join(Join),
    Leave(Leave),
    Like(Like),
    ExtendsOffer(ExtendsOffer),
    ExtendsReject(ExtendsReject),
    Remove(Remove),
    Undo(Undo),
    Update(Update),
    View(View),
    Listen(Listen),
    Read(Read),
    Move(Move),
    Announce(Announce),
    Flag(Flag),
    Dislike(Dislike),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// all activity types
pub enum ExtendsIntransitive {
    ExtendsActivity(ExtendsActivity),
    IntransitiveActivity(IntransitiveActivity),
    Arrive(Arrive),
    Travel(Travel),
    Question(Question),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// all activity types
pub enum ExtendsCollectionPage {
    CollectionPage(CollectionPage),
    OrderedCollectionPage(OrderedCollectionPage),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// all activity types
pub enum ExtendsCollection {
    Collection(Collection),
    OrderedCollection(OrderedCollection),
}
