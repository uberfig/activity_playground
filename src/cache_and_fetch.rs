use std::{borrow::Borrow, collections::HashMap, sync::RwLock, time::Duration};

use url::Url;

use crate::activitystream_objects::core_types::ContextWrap;

const MAX_AGE: std::time::Duration = Duration::from_secs(4);

// const base: i64 = 2;

#[derive(Debug, Clone)]
pub struct DomainRequest {
    pub last_adverse: u64,
    pub adverse_events: u64,
}

#[derive(Debug, Clone)]
pub struct CachedItem<T: Clone> {
    pub item: T,
    pub fetched_at: u64,
}

pub struct Cache {
    pub domains: RwLock<HashMap<String, DomainRequest>>,
    pub outgoing_cache: RwLock<HashMap<String, String>>, //cache of objects being externally requested
    pub fetch: RwLock<HashMap<String, CachedItem<ContextWrap>>>, //cache of objects being fetched
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            domains: RwLock::new(HashMap::new()),
            outgoing_cache: RwLock::new(HashMap::new()),
            fetch: RwLock::new(HashMap::new()),
        }
    }
}

pub async fn get_local_object(id: Url) -> ContextWrap {
    todo!()
}

pub async fn get_object(id: Url, cache: &Cache) -> ContextWrap {
    let cached = {
        let read_lock = cache.fetch.read().unwrap();
        read_lock.get(id.as_str()).cloned()
    };

    if let Some(x) = cached {}

    let client = reqwest::Client::new();
    let client = client.get(id).header("accept", "application/activity+json");

    let response = client.send().await.unwrap();
    let response = response.bytes().await.unwrap();

    let object: Result<ContextWrap, _> = serde_json::from_slice(&response);

    object.unwrap()
}

pub async fn fetch_object(id: Url, test_config: &String, cache: &Cache) -> ContextWrap {
    if let Some(x) = id.domain() {
        if x.eq_ignore_ascii_case(test_config) {
            return get_local_object(id).await;
        }
        return get_object(id, cache).await;
    }

    todo!()
}
