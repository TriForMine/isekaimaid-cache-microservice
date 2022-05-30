use crate::types::Guild;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use ciborium::{de, ser};
use dashmap::DashMap;
use futures_util::StreamExt as _;
use std::io::Cursor;
use std::sync::Arc;

#[post("/guilds/set/{guild_id}")]
pub async fn set_guild(
    path: web::Path<u64>,
    mut body: web::Payload,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Guild = de::from_reader(&mut bytes.as_slice()).unwrap();

    data.guilds.insert(guild_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/guilds/size")]
pub async fn get_guilds_size(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let res = data.guilds.len();

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res, &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/guilds/size/{shard_id}")]
pub async fn get_guilds_size_per_shard(
    path: web::Path<u8>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let shard_id = path.into_inner();

    let res = data
        .guilds
        .iter()
        .filter(|v| v.value().shard_id.unwrap() == shard_id)
        .count();

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res, &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/guilds/{guild_id}/members")]
pub async fn get_guilds_members_size(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();
    let res = data
        .members
        .iter()
        .filter(|v| v.value().guild_id == guild_id)
        .count();

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res, &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/guilds/get/{guild_id}")]
pub async fn get_guild(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    let res = data.guilds.get(&guild_id);

    if let Some(r) = res {
        let mut buff = Cursor::new(Vec::new());
        ser::into_writer(r.value(), &mut buff).unwrap();
        let res = buff.get_ref();

        Ok(HttpResponse::Ok().body(res.clone()))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[get("/guilds/has/{guild_id}")]
pub async fn has_guild(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    let res = data.guilds.contains_key(&guild_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/guilds/delete/{guild_id}")]
pub async fn delete_guild(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    data.guilds.remove(&guild_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/guilds/get")]
pub async fn get_guilds(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    println!("{:?}", data.guilds);

    ser::into_writer(&data.guilds, &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[post("/guilds/set")]
pub async fn set_guilds(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: DashMap<u64, Guild> = de::from_reader(&mut bytes.as_slice()).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input.into_iter().collect::<Vec<_>>(), &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
