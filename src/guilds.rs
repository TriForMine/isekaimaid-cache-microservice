use crate::types::Guild;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use ciborium::{de, ser};
use futures_util::StreamExt as _;
use std::io::Cursor;
use dashmap::DashMap;

#[post("/guilds/set/{guild_id}")]
pub async fn set_guild(
    path: web::Path<u64>,
    mut body: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Guild = de::from_reader(&*bytes).unwrap();

    data.guilds.insert(guild_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/guilds/get/{guild_id}")]
pub async fn get_guild(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    let res = data.guilds.get(&guild_id);

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res.as_deref().unwrap(), &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/guilds/has/{guild_id}")]
pub async fn has_guild(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    let res = data.guilds.contains_key(&guild_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/guilds/delete/{guild_id}")]
pub async fn delete_guild(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let guild_id = path.into_inner();

    data.guilds.remove(&guild_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/guilds/get")]
pub async fn get_guilds(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
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

    let input: DashMap<u64, Guild> = de::from_reader(&*bytes).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input.into_iter().collect::<Vec<_>>(), &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
