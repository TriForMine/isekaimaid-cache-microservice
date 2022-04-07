mod channels;
mod types;

use crate::channels::{delete_channel, get_channel, get_channels, has_channel, set_channel};
use crate::types::Channel;
use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use ciborium::{de, ser};
use dashmap::DashMap;
use futures_util::StreamExt as _;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::Cursor;

pub struct AppState {
    channels: DashMap<u64, Channel>,
}

#[post("/channels/set")]
async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
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
        channels: DashMap::new(),
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
