use crate::types::User;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use ciborium::{de, ser};
use futures_util::StreamExt as _;
use std::io::Cursor;
use dashmap::DashMap;

#[post("/users/set/{user_id}")]
pub async fn set_user(
    path: web::Path<u64>,
    mut body: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: User = de::from_reader(&mut bytes.as_slice()).unwrap();

    data.users.insert(user_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/users/get/{user_id}")]
pub async fn get_user(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let res = data.users.get(&user_id);

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res.as_deref().unwrap(), &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/users/has/{user_id}")]
pub async fn has_user(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let res = data.users.contains_key(&user_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/users/delete/{user_id}")]
pub async fn delete_user(
    path: web::Path<u64>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    data.users.remove(&user_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/users/get")]
pub async fn get_users(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    println!("{:?}", data.users);

    ser::into_writer(&data.users, &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[post("/users/set")]
pub async fn set_users(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: DashMap<u64, User> = de::from_reader(&mut bytes.as_slice()).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input.into_iter().collect::<Vec<_>>(), &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
