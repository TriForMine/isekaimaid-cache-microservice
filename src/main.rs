use std::io::{Cursor};
use std::sync::{Arc};
use futures_util::StreamExt as _;
use ciborium::{de, ser};
use dashmap::DashMap;
use serde_derive::{Serialize, Deserialize};
use actix_web::{post, get, web, App, HttpServer, HttpResponse, Error};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


struct AppState {
    channels: DashMap<u64, Channel>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Channel {
    #[serde(rename = "type")]
    kind: u8,
    name: String,
    guild_id: u64,
    permission_overwrites: Vec<String>,
    id: u64,
}

#[post("/channels/set/{channel_id}")]
async fn set_channel(path: web::Path<u64>, mut body: web::Payload, data: web::Data<AppState>) -> Result<HttpResponse, Error>  {
    let channel_id = path.into_inner();

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: Channel = de::from_reader(&*bytes).unwrap();

    data.channels.insert(channel_id, input);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/channels/get/{channel_id}")]
async fn get_channel(path: web::Path<u64>, data: web::Data<AppState>) -> Result<HttpResponse, Error>  {
    let channel_id = path.into_inner();

    let res = data.channels.get(&channel_id);

    let mut buff = Cursor::new(Vec::new());
    ser::into_writer(&res.as_deref().unwrap(), &mut buff).unwrap();
    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[get("/channels/has/{channel_id}")]
async fn has_channel(path: web::Path<u64>, data: web::Data<AppState>) -> Result<HttpResponse, Error>  {
    let channel_id = path.into_inner();

    let res = data.channels.contains_key(&channel_id);

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[post("/channels/delete/{channel_id}")]
async fn delete_channel(path: web::Path<u64>, data: web::Data<AppState>) -> Result<HttpResponse, Error>  {
    let channel_id = path.into_inner();

    data.channels.remove(&channel_id);

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/channels/get")]
async fn get_channels(data: web::Data<AppState>) -> Result<HttpResponse, Error>  {
    let mut buff = Cursor::new(Vec::new());

    println!("{:?}", data.channels);

    ser::into_writer(&data.channels, &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[post("/channels/set")]
async fn index(mut body: web::Payload) -> Result<HttpResponse, Error>  {
    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let input: DashMap<u64, Channel> = de::from_reader(&*bytes).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input.into_iter().collect::<Vec<_>>(), &mut buff).unwrap();

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let state = web::Data::new(AppState {
        channels: DashMap::new()
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(get_channels)
            .service(set_channel)
            .service(get_channel)
            .service(has_channel)
            .service(delete_channel)
    })
        .bind_openssl("127.0.0.1:9493", builder)?
        .run()
        .await
}
