use crate::types::Message;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use ciborium::{de, ser};
use futures_util::StreamExt as _;
use std::io::Cursor;
use dashmap::DashMap;

#[post("/messages/set/{message_id}")]
pub async fn set_message(
    path: web::Path<u64>,
    mut body: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Message = de::from_reader(&*bytes).unwrap();

    data.messages.insert(message_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/messages/get/{message_id}")]
pub async fn get_message(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    println!("{:?}", data.messages);

    let res = data.messages.get(&message_id);

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res.as_deref().unwrap(), &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/messages/has/{message_id}")]
pub async fn has_message(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    let res = data.messages.contains_key(&message_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/messages/delete/{message_id}")]
pub async fn delete_message(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let message_id = path.into_inner();

    data.messages.remove(&message_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/messages/get")]
pub async fn get_messages(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    println!("{:?}", data.messages);

    ser::into_writer(&data.messages, &mut buff).unwrap();

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

    let input: DashMap<u64, Message> = de::from_reader(&*bytes).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input.into_iter().collect::<Vec<_>>(), &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
