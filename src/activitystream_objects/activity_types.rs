use serde::{Deserialize, Serialize};

use super::core_types::{Activity, IntransitiveActivity};

//--------------------new implimentaition---------------

//--------------Extended Types---------

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A specialization of [`Accept`] indicating that the acceptance is tentative.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept
pub struct TentativeAccept {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has created the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create
pub struct Create {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has deleted the object. If specified,
/// the origin indicates the context from which the object was deleted.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete
pub struct Delete {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is ignoring the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore
pub struct Ignore {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has joined the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join
pub struct Join {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has left the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave
pub struct Leave {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor likes, recommends or endorses the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like
pub struct Like {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is offering the object.
/// If specified, the target indicates the entity to which the object is being offered.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer
pub struct Offer {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A specialization of [`Offer`] in which the actor is extending an invitation for the object to the target.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite
pub struct Invite {
    #[serde(flatten)]
    pub extends_offer: Offer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is rejecting the object.
/// The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject
pub struct Reject {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A specialization of [`Reject`] in which the rejection is considered tentative.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject
pub struct TentativeReject {
    #[serde(flatten)]
    pub extends_reject: Reject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is removing the object.
/// If specified, the origin indicates the context from which the object is being removed.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove
pub struct Remove {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has viewed the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view
pub struct View {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has listened to the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen
pub struct Listen {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has read the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read
pub struct Read {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor has moved object from origin to target.
/// If the origin or target are not specified, either can be determined by context.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move
pub struct Move {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is calling the target's attention the object.
/// The origin typically has no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce
pub struct Announce {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor is blocking the object.
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Indicates that the actor dislikes the object.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-dislike
pub struct Dislike {
    #[serde(flatten)]
    pub extends_activity: Activity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsAccept {
    TentativeAccept(TentativeAccept),
    Accept(Accept),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsOffer {
    Offer(Offer),
    Invite(Invite),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsReject {
    Reject(Reject),
    TentativeReject(TentativeReject),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsIgnore {
    Ignore(Ignore),
    Block(Block),
}
