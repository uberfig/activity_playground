use std::{
    collections::HashMap,
    fmt::format,
    sync::RwLock,
    time::{Duration, SystemTime},
};

use actix_web::web::Data;
use openssl::{pkey::Private, rsa::Rsa};
use url::Url;

use crate::{
    activitystream_objects::{
        actors::{Actor, ActorType, PublicKey},
        core_types::{ActivityStream, ContextWrap, Object},
    },
    db::{account_creation::UserLinks, conn::DbConn},
    protocol::fetch::authorized_fetch,
};

const MAX_AGE: std::time::Duration = Duration::from_secs(40);

const MAX_ADVERSE: i32 = 6;

// const base: i64 = 2;

#[derive(Debug, Clone)]
pub struct DomainRequest {
    pub last_adverse: u64,
    pub adverse_events: u64,
}

#[derive(Debug, Clone)]
pub struct CachedItem<T: Clone> {
    pub item: T,
    pub fetched_at: SystemTime,
}

pub struct InstanceActor {
    pub actor: Actor,
    pub key_id: String,
    pub private_key: Rsa<Private>,
}

fn instance_actor_links(domain: &str) -> UserLinks {
    UserLinks {
        id: format!("https://{domain}/actor"),
        inbox: format!("https://{domain}/actor/inbox"),
        outbox: format!("https://{domain}/actor/outbox"),
        followers: format!("https://{domain}/actor/followers"),
        following: format!("https://{domain}/actor/following"),
        liked: format!("https://{domain}/actor/liked"),
    }
}

impl InstanceActor {
    pub fn new(private_key: Rsa<Private>, public_key_pem: String, domain: &str) -> InstanceActor {
        let links = instance_actor_links(domain);
        let object = Object::new(Url::parse(&links.id).unwrap());
        let public_key = PublicKey {
            id: format!("{}#main-key", &links.id),
            owner: links.id,
            public_key_pem,
        };
        let key_id = public_key.id.clone();
        let actor = Actor {
            type_field: ActorType::Application,
            preferred_username: "bayou.internal".to_owned(),
            extends_object: object,
            public_key: public_key,
            inbox: links.inbox,
            outbox: links.outbox,
            followers: links.followers,
            following: links.following,
            ap_user_id: None,
            domain: None,
            liked: None,
        };

        InstanceActor {
            actor,
            key_id: key_id,
            private_key,
        }
    }
}

pub struct Cache {
    pub instance_actor: InstanceActor,
    pub domains: RwLock<HashMap<String, DomainRequest>>,
    pub outgoing_cache: RwLock<HashMap<String, String>>, //cache of objects being externally requested
    pub fetch: RwLock<HashMap<String, CachedItem<ActivityStream>>>, //cache of objects being fetched
}

impl Cache {
    pub fn new(instance_actor: InstanceActor) -> Cache {
        Cache {
            instance_actor,
            domains: RwLock::new(HashMap::new()),
            outgoing_cache: RwLock::new(HashMap::new()),
            fetch: RwLock::new(HashMap::new()),
        }
    }
}

pub async fn get_local_object(id: Url) -> ActivityStream {
    todo!()
}

pub enum FetchErr {
    MaxAdverse,
    DoesNotExist,
}

pub async fn get_federated_object(
    id: Url,
    cache: &Cache,
    conn: &Data<DbConn>,
) -> Result<ActivityStream, FetchErr> {
    let cached = {
        let read_lock = cache.fetch.read().unwrap();
        read_lock.get(id.as_str()).cloned()
    };

    if let Some(x) = &cached {
        dbg!(x);

        let time = SystemTime::now();
        let elapsed = time.duration_since(x.fetched_at);

        let elapsed = match elapsed {
            Ok(x) => x,
            Err(x) => x.duration(),
        };

        if elapsed.as_secs() > MAX_AGE.as_secs() {
            //get from database, it may have had an update activity or smth
            todo!()
        } else {
            return Ok(x.item.clone());
        }
    }

    let object = authorized_fetch(
        &id,
        &cache.instance_actor.key_id,
        &cache.instance_actor.private_key,
    )
    .await;
    let object = match object {
        Ok(x) => x,
        Err(x) => todo!(),
    };

    let time = SystemTime::now();

    {
        let mut write_lock = cache.fetch.write().unwrap();
        write_lock.insert(
            id.as_str().to_owned(),
            CachedItem {
                item: object.clone(),
                fetched_at: time,
            },
        );
    }

    Ok(object)
}

pub async fn fetch_object(
    id: Url,
    state: &Data<crate::config::Config>,
    cache: &Cache,
    conn: &Data<DbConn>,
) -> Result<ActivityStream, FetchErr> {
    if let Some(x) = id.domain() {
        if x.eq_ignore_ascii_case(&state.instance_domain) {
            return Ok(get_local_object(id).await);
        }
        return get_federated_object(id, cache, conn).await;
    }

    todo!()
}
