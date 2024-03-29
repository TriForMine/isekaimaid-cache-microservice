use crate::types::Message;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use dashmap::DashMap;
use futures_util::StreamExt as _;
use std::io::Cursor;
use std::sync::Arc;

#[post("/messages/set/{message_id}")]
pub async fn set_message(
    path: web::Path<u64>,
    mut body: web::Payload,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Message = cbor4ii::serde::from_reader(&mut bytes.as_slice()).unwrap();

    data.messages.insert(message_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/messages/size")]
pub async fn get_messages_size(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let res = data.messages.len();

    let mut buff = Cursor::new(Vec::new());
    cbor4ii::serde::to_writer(&mut buff, &res).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/messages/get/{message_id}")]
pub async fn get_message(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    let res = data.messages.get(&message_id);

    if let Some(r) = res {
        let mut buff = Cursor::new(Vec::new());
        cbor4ii::serde::to_writer(&mut buff, r.value()).unwrap();
        let res = buff.get_ref();

        Ok(HttpResponse::Ok().body(res.clone()))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[get("/messages/has/{message_id}")]
pub async fn has_message(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    let res = data.messages.contains_key(&message_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/messages/delete/{message_id}")]
pub async fn delete_message(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    data.messages.remove(&message_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/messages/get")]
pub async fn get_messages(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    cbor4ii::serde::to_writer(&mut buff, &data.messages).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[post("/messages/set")]
pub async fn set_messages(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: DashMap<u64, Message> = cbor4ii::serde::from_reader(&mut bytes.as_slice()).unwrap();

    let mut buff = Cursor::new(Vec::new());

    cbor4ii::serde::to_writer(&mut buff, &input.into_iter().collect::<Vec<_>>()).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
