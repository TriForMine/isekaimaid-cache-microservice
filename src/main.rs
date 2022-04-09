mod channels;
mod types;
mod messages;

use crate::channels::{delete_channel, get_channel, get_channels, has_channel, set_channel, set_channels};
use crate::types::{Channel, Message};
use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use ciborium::{de, ser};
use dashmap::DashMap;
use futures_util::StreamExt as _;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::messages::{delete_message, get_message, get_messages, has_message, set_message, set_messages};

pub struct AppState {
    channels: DashMap<u64, Channel>,
    messages: DashMap<u64, Message>,
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
        messages: DashMap::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(set_channels)
            .service(get_channels)
            .service(set_channel)
            .service(get_channel)
            .service(has_channel)
            .service(delete_channel)

            .service(set_messages)
            .service(get_messages)
            .service(set_message)
            .service(get_message)
            .service(has_message)
            .service(delete_message)
    })
    .bind_openssl("127.0.0.1:9493", builder)?
    .run()
    .await
}
