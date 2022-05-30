use crate::types::Member;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use ciborium::{de, ser};
use dashmap::DashMap;
use futures_util::StreamExt as _;
use std::io::Cursor;
use std::sync::Arc;

#[post("/members/set/{member_id}")]
pub async fn set_member(
    path: web::Path<String>,
    mut body: web::Payload,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let member_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Member = de::from_reader(&mut bytes.as_slice()).unwrap();

    data.members.insert(member_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/members/size")]
pub async fn get_members_size(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let res = data.members.len();

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res, &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/members/get/{member_id}")]
pub async fn get_member(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let member_id = path.into_inner();

    let res = data.members.get(&member_id);

    if let Some(r) = res {
        let mut buff = Cursor::new(Vec::new());
        ser::into_writer(r.value(), &mut buff).unwrap();
        let res = buff.get_ref();

        Ok(HttpResponse::Ok().body(res.clone()))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[get("/members/has/{member_id}")]
pub async fn has_member(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let member_id = path.into_inner();

    let res = data.members.contains_key(&member_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/members/delete/{member_id}")]
pub async fn delete_member(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let member_id = path.into_inner();

    data.members.remove(&member_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/members/get")]
pub async fn get_members(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    println!("{:?}", data.members);

    ser::into_writer(&data.members, &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[post("/members/set")]
pub async fn set_members(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: DashMap<String, Member> = de::from_reader(&mut bytes.as_slice()).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input.into_iter().collect::<Vec<_>>(), &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
