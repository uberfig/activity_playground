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
    pub preview: Option<String>, //TODO
    pub rel: Option<String>,     //TODO
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

//--------------Extended Types---------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor accepts the object. The target property
/// can be used in certain circumstances to indicate the context into
/// which the object has been accepted.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept
pub struct Accept {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A specialization of [`Accept`] indicating that the acceptance is tentative.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept
pub struct TentativeAccept {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has added the object to the target.
/// If the target property is not explicitly specified, the target
/// would need to be determined implicitly by context. The origin
/// can be used to identify the context from which the object originated.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add
pub struct Add {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// An [`IntransitiveActivity`] that indicates that the actor has
/// arrived at the location. The origin can be used to identify the
/// context from which the actor originated. The target typically
/// has no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive
pub struct Arrive {
    #[serde(flatten)]
    pub extends_intransitive: IntransitiveActivity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has created the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create
pub struct Create {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has deleted the object. If specified,
/// the origin indicates the context from which the object was deleted.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete
pub struct Delete {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is "following" the object. Following
/// is defined in the sense typically used within Social systems in
/// which the actor is interested in any activity performed by or on
/// the object. The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow
pub struct Follow {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is ignoring the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore
pub struct Ignore {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has joined the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join
pub struct Join {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has left the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave
pub struct Leave {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor likes, recommends or endorses the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like
pub struct Like {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is offering the object.
/// If specified, the target indicates the entity to which the object is being offered.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer
pub struct Offer {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A specialization of [`Offer`] in which the actor is extending an invitation for the object to the target.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite
pub struct Invite {
    #[serde(flatten)]
    pub extends_offer: Offer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is rejecting the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject
pub struct Reject {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A specialization of [`Reject`] in which the rejection is considered tentative.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject
pub struct TentativeReject {
    #[serde(flatten)]
    pub extends_reject: Reject,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is removing the object.
/// If specified, the origin indicates the context from which the object is being removed.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove
pub struct Remove {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is undoing the object. In most cases,
/// the object will be an [`Activity`] describing some previously performed action (for instance,
/// a person may have previously "liked" an article but, for whatever reason,
/// might choose to undo that like at some later point in time).
///
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-undo
pub struct Undo {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has updated the object.
/// Note, however, that this vocabulary does not define a mechanism for
/// describing the actual set of modifications made to object.
///
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update
pub struct Update {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has viewed the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view
pub struct View {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has listened to the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen
pub struct Listen {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has read the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read
pub struct Read {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has moved object from origin to target.
/// If the origin or target are not specified, either can be determined by context.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move
pub struct Move {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is traveling to target from origin.
/// Travel is an IntransitiveObject whose actor specifies the direct object.
/// If the target or origin are not specified, either can be determined by context.  
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel
pub struct Travel {
    #[serde(flatten)]
    pub extends_intransitive: IntransitiveActivity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is calling the target's attention the object.
/// The origin typically has no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce
pub struct Announce {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
///	Indicates that the actor is blocking the object.
/// Blocking is a stronger form of [`Ignore`].
/// The typical use is to support social systems that allow one user
/// to block activities or content of other users.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block
pub struct Block {
    #[serde(flatten)]
    pub extends_ignore: Ignore,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is "flagging" the object.
/// Flagging is defined in the sense common to many social platforms
/// as reporting content as being inappropriate for any number of reasons.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-flag
pub struct Flag {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor dislikes the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-dislike
pub struct Dislike {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Represents a question being asked.
/// Question objects are an extension of [`IntransitiveActivity`]. That is,
/// the Question object is an Activity, but the direct object is the question
/// itself and therefore it would not contain an object property.
///
/// Either of the anyOf and oneOf properties MAY be used to express possible answers,
/// but a Question object MUST NOT have both properties.
///
/// Commonly used for polls
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-question
pub struct Question {
    #[serde(flatten)]
    pub extends_intransitive: IntransitiveActivity,
    pub one_of: Option<String>, //TODO
    pub any_of: Option<String>, //TODO
    pub closed: Option<String>, //TODO
}

//-------------inheritance heiarchy-------

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsAccept {
    TentativeAccept(TentativeAccept),    
    Accept(Accept),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsOffer {
    Offer(Offer),
    Invite(Invite),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsReject {
    Reject(Reject),
    TentativeReject(TentativeReject),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsIgnore {
    Ignore(Ignore),
    Block(Block),
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

//-------------------glue--------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ExtendsObject {
    Object(Object),
    ExtendsIntransitive(ExtendsIntransitive),   
    ExtendsCollection(ExtendsCollection),
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
