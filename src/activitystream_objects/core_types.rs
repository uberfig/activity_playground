use serde::{Deserialize, Serialize};

use super::activity_types::*;

//-------------------glue--------------
#[derive(Serialize, Deserialize, Debug)]
/// wraps activitystream to include context
pub struct ContextWrap {
    #[serde(flatten)]
    pub context: Context,
    #[serde(flatten)]
    pub activity_stream: ActivityStream,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Context {
    #[serde(rename = "@context")]
    Array(Vec<String>),
    #[serde(rename = "@context")]
    Single(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
// #[serde(tag = "type")]
pub enum ActivityStream {
    ExtendsObject(Box<ExtendsObject>),
    LinkType(LinkType),
}

//--------------primitive-----------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ObjectWrapper {
    Object(Object),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum LinkWrapper {
    Link(Link),
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
    Expanded(LinkWrapper),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// represents a field that could be an object or a link
pub enum RangeLinkObject {
    Object(Box<ExtendsObject>),
    Link(LinkType),
}

//---------------Activities--------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum IntransitiveActivityWrapper {
    IntransitiveActivity(IntransitiveActivity),
}

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
#[serde(tag = "type")]
pub enum ActivityWrapper {
    Activity(Activity),
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
#[serde(tag = "type")]
pub enum CollectionyWrapper {
    Collection(Collection),
}

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
#[serde(tag = "type")]
pub enum OrderedCollectionWrapper {
    OrderedCollection(OrderedCollection),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(flatten)]
    pub extends_collection: Collection,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CollectionPageWrapper {
    CollectionPage(CollectionPage),
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
#[serde(tag = "type")]
pub enum OrderedCollectionPageWrapper {
    OrderedCollectionPage(OrderedCollectionPage),
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
    Object(ObjectWrapper),
    ExtendsIntransitive(ExtendsIntransitive),
    ExtendsCollection(ExtendsCollection),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsActivity {
    Activity(ActivityWrapper),
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
    IntransitiveActivity(IntransitiveActivityWrapper),
    Arrive(Arrive),
    Travel(Travel),
    Question(Question),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// all activity types
pub enum ExtendsCollectionPage {
    CollectionPage(Box<CollectionPageWrapper>),
    OrderedCollectionPage(Box<OrderedCollectionPageWrapper>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// all activity types
pub enum ExtendsCollection {
    Collection(Collection),
    OrderedCollection(OrderedCollection),
    ExtendsCollectionPage
}
