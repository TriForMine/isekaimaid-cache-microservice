use crate::types::Channel;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use ciborium::{de, ser};
use futures_util::StreamExt as _;
use std::io::Cursor;
use std::ops::Deref;
use std::sync::Arc;
use dashmap::DashMap;

#[post("/channels/set/{channel_id}")]
pub async fn set_channel(
    path: web::Path<u64>,
    mut body: web::Payload,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let channel_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Channel = de::from_reader(&mut bytes.as_slice()).unwrap();

    data.channels.insert(channel_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/channels/size")]
pub async fn get_channels_size(
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let res = data.channels.len();

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res, &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/channels/get/{channel_id}")]
pub async fn get_channel(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let channel_id = path.into_inner();

    let res = data.channels.get(&channel_id);

    if let Some(r) = res {
        let mut buff = Cursor::new(Vec::new());
        ser::into_writer(&r.deref(), &mut buff).unwrap();
        let res = buff.get_ref();

        Ok(HttpResponse::Ok().body(res.clone()))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[get("/channels/has/{channel_id}")]
pub async fn has_channel(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let channel_id = path.into_inner();

    let res = data.channels.contains_key(&channel_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/channels/delete/{channel_id}")]
pub async fn delete_channel(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let channel_id = path.into_inner();

    data.channels.remove(&channel_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/channels/get")]
pub async fn get_channels(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    println!("{:?}", data.channels);

    ser::into_writer(&data.channels, &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[post("/channels/set")]
pub async fn set_channels(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: DashMap<u64, Channel> = de::from_reader(&mut bytes.as_slice()).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input.into_iter().collect::<Vec<_>>(), &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
