use serde::{Deserialize, Serialize};
use url::Url;

use super::{activity_types::*, actors::Actor};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityStream {
    #[serde(flatten)]
    pub content: ContextWrap,
}

impl ActivityStream {
    pub fn get_actor(self) -> Option<Box<Actor>> {
        match self.content.activity_stream {
            RangeLinkObject::Object(ExtendsObject::Actor(x)) => Some(x),
            _ => None,
        }
    }
}

//-------------------glue--------------
#[derive(Serialize, Deserialize, Debug, Clone)]
/// wraps base object to include context
pub struct ContextWrap {
    #[serde(flatten)]
    pub context: Context,
    #[serde(flatten)]
    pub activity_stream: RangeLinkObject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Context {
    #[serde(rename = "@context")]
    Array(Vec<String>),
    #[serde(rename = "@context")]
    Single(String),
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(untagged)]
// pub enum ActivityStream {
//     ExtendsObject(Box<ExtendsObject>),
//     LinkType(Box<LinkType>),
// }

//--------------------inheritence---------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsObject {
    Object(Box<ObjectWrapper>),
    ExtendsIntransitive(Box<ExtendsIntransitive>),
    ExtendsCollection(Box<ExtendsCollection>),
    Actor(Box<Actor>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsActivity {
    Activity(Activity),
    // ExtendsAccept(ExtendsAccept),
    // Add(Add),
    // Create(Create),
    // Delete(Delete),
    // Follow(Follow),
    // ExtendsIgnore(ExtendsIgnore),
    // Join(Join),
    // Leave(Leave),
    // Like(Like),
    // ExtendsOffer(ExtendsOffer),
    // ExtendsReject(ExtendsReject),
    // Remove(Remove),
    // Undo(Undo),
    // Update(Update),
    // View(View),
    // Listen(Listen),
    // Read(Read),
    // Move(Move),
    // Announce(Announce),
    // Flag(Flag),
    // Dislike(Dislike),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsIntransitive {
    ExtendsActivity(ExtendsActivity),
    IntransitiveActivity(IntransitiveActivity),
    // Arrive(Arrive),
    // Travel(Travel),
    Question(Question),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsCollectionPage {
    CollectionPage(Box<CollectionPageWrapper>),
    OrderedCollectionPage(Box<OrderedCollectionPageWrapper>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsCollection {
    Collection(Collection),
    ExtendsCollectionPage,
}

//--------------primitive-----------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    #[serde(rename = "text/html")]
    Html,
    #[serde(rename = "text/markdown")]
    Markdown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LinkOrArray {
    Single(LinkType),
    Multiple(Vec<LinkType>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RangeLinkObjOrArray {
    Single(RangeLinkObject),
    Multiple(Vec<RangeLinkObject>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ObjectWrapper {
    Object(Object),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ID {
    pub id: Url,
}

impl From<Url> for ID {
    fn from(value: Url) -> Self {
        ID { id: value }
    }
}

impl ID {
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }
    pub fn domain(&self) -> Option<&str> {
        self.id.domain()
    }
}

impl Into<Url> for ID {
    fn into(self) -> Url {
        self.id
    }
}

impl Default for ID {
    fn default() -> Self {
        Self {
            id: Url::parse("invalid").unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(flatten)]
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_to: Option<Box<ExtendsObject>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies the entity (e.g. an application) that generated the object
    pub generator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<RangeLinkObject>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<RangeLinkObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<xsd_types::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Box<ExtendsCollection>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<xsd_types::DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<xsd_types::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<LinkOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the private primary audience of this Object.
    pub bto: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the public secondary audience of this Object.
    pub cc: Option<RangeLinkObjOrArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies one or more Objects that are part of the private secondary audience of this Object.
    pub bcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl Object {
    pub fn new(id: Url) -> Object {
        Object {
            id: ID { id },
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum LinkWrapper {
    Link(Link),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: Url,
    pub hreflang: Option<String>,
    pub media_type: String,
    pub name: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub preview: Option<String>, //TODO
    pub rel: Option<String>,     //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LinkType {
    Simple(Url),
    Expanded(LinkWrapper),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be an object or a link
pub enum RangeLinkObject {
    Object(ExtendsObject),
    Link(Box<LinkType>),
}

//---------------Activities--------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IntransitiveType {
    IntransitiveActivity,
    /// An [`IntransitiveActivity`] that indicates that the actor has
    /// arrived at the location. The origin can be used to identify the
    /// context from which the actor originated. The target typically
    /// has no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive
    Arrive,
    /// Indicates that the actor is traveling to target from origin.
    /// Travel is an IntransitiveObject whose actor specifies the direct object.
    /// If the target or origin are not specified, either can be determined by context.  
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel
    Travel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntransitiveActivity {
    #[serde(rename = "type")]
    pub type_field: IntransitiveType,

    #[serde(flatten)]
    pub extends_object: Object,
    pub actor: Option<String>,      //TODO
    pub target: Option<String>,     //TODO
    pub result: Option<String>,     //TODO
    pub origin: Option<String>,     //TODO
    pub instrument: Option<String>, //TODO
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(tag = "type")]
// pub enum ActivityWrapper {
//     Activity(Activity),
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivityType {
    Activity,
    /// Indicates that the actor accepts the object. The target property
    /// can be used in certain circumstances to indicate the context into
    /// which the object has been accepted.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept
    Accept,
    /// A specialization of [`Accept`] indicating that the acceptance is tentative.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept
    TentativeAccept,
    /// Indicates that the actor has added the object to the target.
    /// If the target property is not explicitly specified, the target
    /// would need to be determined implicitly by context. The origin
    /// can be used to identify the context from which the object originated.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add
    Add,
    /// Indicates that the actor has created the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create
    Create,
    /// Indicates that the actor has deleted the object. If specified,
    /// the origin indicates the context from which the object was deleted.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete
    Delete,
    /// Indicates that the actor is "following" the object. Following
    /// is defined in the sense typically used within Social systems in
    /// which the actor is interested in any activity performed by or on
    /// the object. The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow
    Follow,
    /// Indicates that the actor is ignoring the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore
    Ignore,
    /// Indicates that the actor has joined the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join
    Join,
    /// Indicates that the actor has left the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave
    Leave,
    /// Indicates that the actor likes, recommends or endorses the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like
    Like,
    /// Indicates that the actor is offering the object.
    /// If specified, the target indicates the entity to which the object is being offered.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer
    Offer,
    /// A specialization of [`Offer`] in which the actor is extending an invitation for the object to the target.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite
    Invite,
    /// Indicates that the actor is rejecting the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject
    Reject,
    /// A specialization of [`Reject`] in which the rejection is considered tentative.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject
    TentativeReject,
    /// Indicates that the actor is removing the object.
    /// If specified, the origin indicates the context from which the object is being removed.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove
    Remove,
    /// Indicates that the actor is undoing the object. In most cases,
    /// the object will be an [`Activity`] describing some previously performed action (for instance,
    /// a person may have previously "liked" an article but, for whatever reason,
    /// might choose to undo that like at some later point in time).
    ///
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-undo
    Undo,
    /// Indicates that the actor has updated the object.
    /// Note, however, that this vocabulary does not define a mechanism for
    /// describing the actual set of modifications made to object.
    ///
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update
    Update,
    /// Indicates that the actor has viewed the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view
    View,
    /// Indicates that the actor has listened to the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen
    Listen,
    /// Indicates that the actor has read the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read
    Read,
    /// Indicates that the actor has moved object from origin to target.
    /// If the origin or target are not specified, either can be determined by context.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move
    Move,
    /// Indicates that the actor is calling the target's attention the object.
    /// The origin typically has no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce
    Announce,
    /// Indicates that the actor is blocking the object.
    /// Blocking is a stronger form of [`Ignore`].
    /// The typical use is to support social systems that allow one user
    /// to block activities or content of other users.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block
    Block,
    /// Indicates that the actor is "flagging" the object.
    /// Flagging is defined in the sense common to many social platforms
    /// as reporting content as being inappropriate for any number of reasons.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-flag
    Flag,
    /// Indicates that the actor dislikes the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-dislike
    Dislike,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "type")]
    pub type_field: ActivityType,

    pub object: Option<Box<Object>>,

    #[serde(flatten)]
    pub extends_object: Object,
    pub actor: Option<String>,      //TODO
    pub target: Option<String>,     //TODO
    pub result: Option<String>,     //TODO
    pub origin: Option<String>,     //TODO
    pub instrument: Option<String>, //TODO
}

// --------------collections----------------

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(tag = "type")]
// pub enum CollectionyWrapper {
//     Collection(Collection),
// }
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CollectionType {
    Collection,
    OrderedCollection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "type")]
    pub type_field: CollectionType,
    #[serde(flatten)]
    pub extends_object: Object,
    pub total_items: u32,
    pub current: Option<String>, //TODO
    pub first: Option<String>,   //TODO
    pub last: Option<String>,    //TODO
    pub items: Option<String>,   //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum CollectionPageWrapper {
    CollectionPage(CollectionPage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(flatten)]
    pub extends_collection: Collection,
    pub part_of: Option<String>, //TODO
    pub next: Option<String>,    //TODO
    pub prev: Option<String>,    //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OrderedCollectionPageWrapper {
    OrderedCollectionPage(OrderedCollectionPage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub extends_collection: Collection,
    #[serde(flatten)]
    pub extends_collection_page: CollectionPage,
}
