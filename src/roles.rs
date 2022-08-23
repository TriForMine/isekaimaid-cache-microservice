use crate::types::Role;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use dashmap::DashMap;
use futures_util::StreamExt as _;
use std::io::Cursor;
use std::sync::Arc;

#[post("/roles/set/{role_id}")]
pub async fn set_role(
    path: web::Path<u64>,
    mut body: web::Payload,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let role_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Role = cbor4ii::serde::from_reader(&mut bytes.as_slice()).unwrap();

    data.roles.insert(role_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/roles/size")]
pub async fn get_roles_size(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let res = data.roles.len();

    let mut buff = Cursor::new(Vec::new());
    cbor4ii::serde::to_writer(&mut buff, &res).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/roles/get/{role_id}")]
pub async fn get_role(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let role_id = path.into_inner();

    let res = data.roles.get(&role_id);

    if let Some(r) = res {
        let mut buff = Cursor::new(Vec::new());
        cbor4ii::serde::to_writer(&mut buff, r.value()).unwrap();
        let res = buff.get_ref();

        Ok(HttpResponse::Ok().body(res.clone()))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[get("/roles/has/{role_id}")]
pub async fn has_role(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let role_id = path.into_inner();

    let res = data.roles.contains_key(&role_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/roles/delete/{role_id}")]
pub async fn delete_role(
    path: web::Path<u64>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let role_id = path.into_inner();

    let (role_id, role) = data.roles.remove(&role_id).unwrap();

    for mut member in data.members.iter_mut().filter(|member| member.value().guild_id == role.guild_id) {
        member.value_mut().roles.retain(|&r| r != role_id);
    }

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/roles/get")]
pub async fn get_roles(data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    cbor4ii::serde::to_writer(&mut buff, &data.roles).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[post("/roles/set")]
pub async fn set_roles(
    data: web::Data<Arc<AppState>>,
    mut body: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: DashMap<u64, Role> = cbor4ii::serde::from_reader(&mut bytes.as_slice()).unwrap();

    for role in input.iter() {
        let id = role.key();
        let r = role.value();

        data.roles.insert(*id, *r);
    }

    Ok(HttpResponse::Ok().body("Ok"))
}
