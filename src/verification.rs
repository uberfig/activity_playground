use std::collections::HashMap;

use actix_web::{web, HttpRequest};
use openssl::hash::MessageDigest;
use serde::{Deserialize, Serialize};

use crate::activitystream_objects::VerificationActor;

pub fn generate_digest(body: &[u8]) -> String {
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha256()).unwrap();
    hasher.update(body).unwrap();
    let digest: &[u8] = &hasher.finish().unwrap();

    //digest_base64
    openssl::base64::encode_block(digest)
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum RequestVerificationError {
    NoMessageDigest,
    BadMessageDigest,
    BadMessageBody,
    DigestDoesNotMatch,
    NoMessageSignature,
    BadMessageSignature,
    NoSignatureKey,
    NoSignature,
    SignatureIncorrectBase64,
    ActorFetchFailed,
    ActorFetchBodyFailed,
    ActorDeserializeFailed,
    NoSignatureHeaders,
    SignatureVerifyFailed,
    NoDate,
}

///verifys a request and returns the message body if its valid
pub async fn verify_request(
    request: HttpRequest,
    body: web::Bytes,
    path: &str,
    instance_domain: &str,
) -> Result<String, RequestVerificationError> {
    let request_headers = request.headers();

    //check digest matches

    let Some(digest) = request_headers.get("Digest") else {
        // return Ok(HttpResponse::Unauthorized().body("no message digest"));
        return Err(RequestVerificationError::NoMessageDigest);
    };

    let Ok(digest) = String::from_utf8(digest.as_bytes().to_vec()) else {
        // return Ok(HttpResponse::Unauthorized().body("bad message digest"));
        return Err(RequestVerificationError::BadMessageDigest);
    };

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        // return Ok(HttpResponse::Unauthorized().body("bad message body"));
        return Err(RequestVerificationError::BadMessageBody);
    };

    let generated_digest = "SHA-256=".to_owned() + &generate_digest(body.as_bytes());

    if !digest.eq(&generated_digest) {
        // return Ok(HttpResponse::Unauthorized().body("digest does not match body"));
        return Err(RequestVerificationError::DigestDoesNotMatch);
    }

    //get the signature header

    let Some(signature_header) = request_headers.get("Signature") else {
        // return Ok(HttpResponse::Unauthorized().body("no message signature"));
        return Err(RequestVerificationError::NoMessageSignature);
    };

    let Ok(signature_header) = String::from_utf8(signature_header.as_bytes().to_vec()) else {
        // return Ok(HttpResponse::Unauthorized().body("bad message signature"));
        return Err(RequestVerificationError::BadMessageSignature);
    };

    let signature_header: HashMap<String, String> = signature_header
        .split(',')
        .filter_map(|pair| {
            pair.split_once('=').map(|(key, value)| {
                (
                    key.replace("/[^A-Za-z]/", ""),
                    value.replace("/[^A-Za-z]/", ""),
                )
            })
        })
        .collect();

    let Some(key_id) = signature_header.get("keyId") else {
        // return Ok(HttpResponse::Unauthorized().body("no signature key id provided"));
        return Err(RequestVerificationError::NoSignatureKey);
    };
    let key_id = key_id.replace('"', "");

    let Some(signature) = signature_header.get("signature") else {
        // return Ok(HttpResponse::Unauthorized().body("no signature provided in signature header"));
        return Err(RequestVerificationError::NoSignature);
    };
    let signature = signature.replace('"', "");

    dbg!(&signature);

    // let Ok(signature) = openssl::base64::decode_block(&signature) else {
    //     // return Ok(HttpResponse::Unauthorized().body("signature is incorrectly base64 encoded"));
    //     return Err(RequestVerificationError::SignatureIncorrectBase64);
    // };

    // dbg!(&key_id);

    let client = reqwest::Client::new();
    let client = client
        .get(key_id)
        .header("accept", "application/activity+json");

    // let test =client.send().await;
    // dbg!(&test);

    // let Ok(res) = test else {
    //     return Err(RequestVerificationError::ActorFetchFailed);
    // };

    let Ok(res) = client.send().await else {
        // return Ok(HttpResponse::Unauthorized().body("unable to fetch key"));
        return Err(RequestVerificationError::ActorFetchFailed);
    };
    // dbg!(&res);

    let Ok(actor) = res.bytes().await else {
        // return Ok(HttpResponse::Unauthorized().body("failed to fetch key body"));
        return Err(RequestVerificationError::ActorFetchBodyFailed);
    };

    let actor: Result<VerificationActor, _> = serde_json::from_slice(&actor);
    // dbg!(&actor);
    let Ok(actor) = actor else {
        // return Ok(HttpResponse::Unauthorized().body("actor owning key could not be deserialized"));
        return Err(RequestVerificationError::ActorDeserializeFailed);
    };

    let key =
        openssl::rsa::Rsa::public_key_from_pem(actor.public_key.public_key_pem.as_bytes()).unwrap();

    let Some(headers) = signature_header.get("headers") else {
        // return Ok(HttpResponse::Unauthorized().body("no signature headers provided"));
        return Err(RequestVerificationError::NoSignatureHeaders);
    };

    let Some(_) = request_headers.get("date") else {
        return Err(RequestVerificationError::NoDate);
    };

    // let date = String::from_utf8(date.as_bytes().to_vec()).unwrap();

    //generate a sign string of the actual request's headers with the real header values mentoned in the provided sign string
    // let comparison_string= format!("(request-target): post /inbox\nhost: {instance_domain}\ndate: {date}\ndigest: SHA-256={digest}");

    let comparison_string: Vec<String> = headers
        .replace('"', "")
        .split(' ')
        .map(|signed_header_name| {
            // if signed_header_name.eq("(request-target)") {
            //     format!("(request-target): post {path}")
            // }
            // else if  {

            // }
            match signed_header_name {
                "(request-target)" => {
                    format!("(request-target): post {path}")
                }
                "host" => {
                    format!("host: {instance_domain}")
                }
                _ => {
                    let value = String::from_utf8(
                        request_headers
                            .get(signed_header_name)
                            .unwrap()
                            .as_bytes()
                            .to_vec(),
                    )
                    .unwrap();
                    let x = format!("{signed_header_name}: {value}",);
                    dbg!(&x);
                    x
                }
            }

            // else {
            //     // let mut capatalized: Vec<char> = signed_header_name.chars().collect();
            //     // capatalized[0] = capatalized[0].to_uppercase().nth(0).unwrap();
            //     // let capatalized: String = capatalized.into_iter().collect();
            //     // dbg!(&capatalized);
            //     // println!("{capatalized}");
            //     // let capitalized = String::from_utf8(
            //     //     request_headers
            //     //         .get(capatalized)
            //     //         .unwrap()
            //     //         .as_bytes()
            //     //         .to_vec()
            //     // )
            //     // .unwrap();

            //     let value = String::from_utf8(
            //         request_headers
            //             .get(signed_header_name)
            //             .unwrap()
            //             .as_bytes()
            //             .to_vec(),
            //     )
            //     .unwrap();
            //     let x = format!("{signed_header_name}: {value}",);
            //     dbg!(&x);
            //     x
            // }
        })
        .collect();

    let comparison_string = comparison_string.join("\n");
    dbg!(&comparison_string);

    let pubkey = openssl::pkey::PKey::from_rsa(key).unwrap();

    let mut verifier =
        openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pubkey).unwrap();
    let input = &comparison_string;
    // dbg!(&input);
    // let input = input.unwrap();
    // let input = &input.into_iter().collect().unwrap();
    verifier.update(input.as_bytes()).unwrap();

    let signature = openssl::base64::decode_block(&signature).unwrap();
    let accepted = verifier.verify(&signature).unwrap();

    if !accepted {
        // return Ok(HttpResponse::Unauthorized().body("Request signature could not be verified"));
        return Err(RequestVerificationError::SignatureVerifyFailed);
    }

    Ok(body)

    // Ok(HttpResponse::Ok()
    //     .status(StatusCode::OK)
    //     .body("OK".to_string())) // <- send response
}
