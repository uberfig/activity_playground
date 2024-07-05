use std::{env, sync::Mutex};

use activity_playground::{
    activitystream_objects::{
        activities::{ChoiceType, IntransitiveActivity, Question, QuestionOption},
        core_types::ActivityStream,
        object::{Object, ObjectType},
    },
    api::{
        // activities::{get_activity, get_object},
        actor::{create_test, get_actor, get_instance_actor},
        inbox::{inspect_inbox, private_inbox, shared_inbox, Inbox},
        objects::get_object,
        outbox::{self, create_post, private_outbox},
        webfinger::webfinger,
    },
    cache_and_fetch::Cache,
    config::Config,
    db::{conn::DbConn, instance_actor::init_instance_actpr},
    protocol::{fetch::authorized_fetch, instance_actor::InstanceActor},
};
use actix_web::{
    // error::ErrorBadRequest,
    get,
    web::{self, Data},
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};
// use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, query};
use url::Url;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/@{preferred_username}")]
async fn get_profile_page(/*conn: Data<DbConn>, */ path: web::Path<String>) -> Result<String> {
    // let val = sqlx::query!(
    //     "INSERT INTO internal_users (password, preferredUsername) VALUES ($1, $2)",
    //     "hi".to_string(),
    //     "hi".to_string()
    // )
    // .execute(&conn.db)
    // .await;

    let preferred_username = path.into_inner();
    Ok(preferred_username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env::set_var("RUST_BACKTRACE", "1");

    let test_obj = Object::new(Url::parse("https://test.com").unwrap())
        .content(Some("hello".to_string()))
        .published_milis(1720121686859)
        .attributed_to_link(Some(Url::parse("https://test.com").unwrap()))
        .to_public()
        .wrap(ObjectType::Note)
        .to_create_activitystream();

    println!("{}", serde_json::to_string_pretty(&test_obj).unwrap());

    let test_create = r#"
    {
        "@context": [
          "https://www.w3.org/ns/activitystreams",
          {
            "ostatus": "http://ostatus.org#",
            "atomUri": "ostatus:atomUri",
            "inReplyToAtomUri": "ostatus:inReplyToAtomUri",
            "conversation": "ostatus:conversation",
            "sensitive": "as:sensitive",
            "toot": "http://joinmastodon.org/ns#",
            "votersCount": "toot:votersCount"
          }
        ],
        "id": "https://mastodon.social/users/ivy_test/statuses/112729853770309074/activity",
        "type": "Create",
        "actor": "https://mastodon.social/users/ivy_test",
        "published": "2024-07-04T19:24:19Z",
        "to": [
          "https://www.w3.org/ns/activitystreams#Public"
        ],
        "cc": [
          "https://mastodon.social/users/ivy_test/followers",
          "https://place.ivytime.gay/users/superivy"
        ],
        "object": {
          "id": "https://mastodon.social/users/ivy_test/statuses/112729853770309074",
          "type": "Note",
          "summary": null,
          "inReplyTo": null,
          "published": "2024-07-04T19:24:19Z",
          "url": "https://mastodon.social/@ivy_test/112729853770309074",
          "attributedTo": "https://mastodon.social/users/ivy_test",
          "to": [
            "https://www.w3.org/ns/activitystreams#Public"
          ],
          "cc": [
            "https://mastodon.social/users/ivy_test/followers",
            "https://place.ivytime.gay/users/superivy"
          ],
          "sensitive": false,
          "atomUri": "https://mastodon.social/users/ivy_test/statuses/112729853770309074",
          "inReplyToAtomUri": null,
          "conversation": "tag:mastodon.social,2024-07-04:objectId=744789693:objectType=Conversation",
          "content": "<p><span class=\"h-card\" translate=\"no\"><a href=\"https://place.ivytime.gay/users/superivy\" class=\"u-url mention\">@<span>superivy</span></a></span> test</p>",
          "contentMap": {
            "en": "<p><span class=\"h-card\" translate=\"no\"><a href=\"https://place.ivytime.gay/users/superivy\" class=\"u-url mention\">@<span>superivy</span></a></span> test</p>"
          },
          "attachment": [],
          "tag": [
            {
              "type": "Mention",
              "href": "https://place.ivytime.gay/users/superivy",
              "name": "@superivy@place.ivytime.gay"
            }
          ],
          "replies": {
            "id": "https://mastodon.social/users/ivy_test/statuses/112729853770309074/replies",
            "type": "Collection",
            "first": {
              "type": "CollectionPage",
              "next": "https://mastodon.social/users/ivy_test/statuses/112729853770309074/replies?only_other_accounts=true&page=true",
              "partOf": "https://mastodon.social/users/ivy_test/statuses/112729853770309074/replies",
              "items": []
            }
          }
        },
        "signature": {
          "type": "RsaSignature2017",
          "creator": "https://mastodon.social/users/ivy_test#main-key",
          "created": "2024-07-04T19:24:20Z",
          "signatureValue": "limnBg+npozgyODp5mK6WRwMR9KBjo7K4bcfVs3wXauGs3C0R7u1ologX3eAR2f5I3WtyrOajY4PEjICAa3MdZ87+Ma6vRbv9he/kkJqbbdiPQMorZt8wybkoTsEGerohcFJviWsz0HbNyxhX2y+TR4TGHqTCYjzrErXakILXdAQ3AGbZe8Ay2fePj0Mxzl4hb42ytdrbRlSBRXcFoT6gwEiJpDXGXbJUl5EI3vFtdAp8Jzaoe6le2yMXsF5UjD8trOySNfY9hs2ct7EeaEg+B5MJ38dlMV0tDpr+iqcQ9mTAYKcQtDb92mWpJLQX5U4tPl60BzSSaQuHa5Y7IpTWg=="
        }
      }"#;

    let deserialized: ActivityStream = serde_json::from_str(&test_create).unwrap();
    dbg!(deserialized);

    let deserialized: ActivityStream = serde_json::from_str(
        r#"{
            "@context": ["https://www.w3.org/ns/activitystreams"],
            "type": "Question",
            "id": "https://example.com",
            "name": "What is the answer?",
            "actor": "https://example.com",
            "anyOf": [
              {
                "type": "Note",
                "name": "Option A"
              },
              {
                "type": "Note",
                "name": "Option B"
              }
            ]
          }"#,
    )
    .unwrap();

    // let deserialized: ActivityStream = serde_json::from_str(
    //     r#"{"type":"Object","@context":["https://context1.com","https://context2.com"],"id":"hi","name":"hi"}"#,
    // )
    // .unwrap();
    // dbg!(&deserialized);

    let test = serde_json::to_string_pretty(&deserialized).unwrap();

    // println!("{test}");
    //----------------config file settings----------------

    let settings = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("gater_config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::default())
        .build();

    let settings = match settings {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{:#?}", x);
            return Ok(());
        }
    };

    let config = match settings.try_deserialize::<Config>() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{:#?}", error);
            return Ok(());
        }
    };

    let bind = config.bind_address.clone();
    let port = config.port;

    //-------------database ------------------

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Error building a connection pool");

    //-------------init instance actor----------------

    let instance_actor = init_instance_actpr(
        &mut pool.begin().await.expect("failed to establish transaction"),
        &config.instance_domain,
    )
    .await;

    // let instance_actor = query!(r#"SELECT * FROM instance_actor LIMIT 1"#,)
    //     .fetch_optional(&pool)
    //     .await;

    // let instance_actor = match instance_actor.unwrap() {
    //     Some(x) => InstanceActor::new(
    //         openssl::rsa::Rsa::private_key_from_pem(x.private_key.as_bytes()).unwrap(),
    //         x.public_key_pem,
    //         &config.instance_domain,
    //     ),
    //     None => {
    //         let rsa = openssl::rsa::Rsa::generate(2048).unwrap();
    //         let private_key = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
    //         let public = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();

    //         let val = query!(
    //             r#"INSERT INTO instance_actor
    //                 (private_key, public_key_pem)
    //             VALUES
    //                 ($1, $2)
    //             "#,
    //             &private_key,
    //             &public,
    //         )
    //         .execute(&pool)
    //         .await;

    //         val.unwrap();
    //         InstanceActor::new(
    //             openssl::rsa::Rsa::private_key_from_pem(private_key.as_bytes()).unwrap(),
    //             public,
    //             &config.instance_domain,
    //         )
    //     }
    // };

    //-------------------------------------------------

    let inbox = Data::new(Inbox {
        inbox: Mutex::new(Vec::new()),
    });

    let cache = Data::new(Cache::new(instance_actor, config.clone()));

    //

    // let test = authorized_fetch(
    //     &Url::parse("https://mastodon.social/users/ivy_test").unwrap(),
    //     &cache.instance_actor.item.key_id,
    //     &cache.instance_actor.item.private_key,
    // )
    // .await;
    // dbg!(&test);
    // test.unwrap();
    //

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(DbConn { db: pool.clone() }))
            .app_data(Data::new(config.to_owned()))
            .app_data(inbox.clone())
            .app_data(cache.clone())
            .service(hello)
            .service(webfinger)
            .service(get_actor)
            .service(get_profile_page)
            // .service(get_activity)
            // .service(get_object)
            .service(create_test)
            // .service(post_test)
            .service(shared_inbox)
            .service(private_inbox)
            .service(inspect_inbox)
            .service(create_post)
            .service(private_outbox)
            .service(get_object)
            .service(get_instance_actor)
    })
    .bind((bind, port))?
    .run()
    .await
}
