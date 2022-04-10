mod channels;
mod types;
mod messages;
mod guilds;

use crate::channels::{delete_channel, get_channel, get_channels, has_channel, set_channel, set_channels};
use crate::types::{Channel, Guild, Message};
use actix_web::{web, App, HttpServer};
use dashmap::DashMap;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::guilds::{delete_guild, get_guild, get_guilds, has_guild, set_guild, set_guilds};
use crate::messages::{delete_message, get_message, get_messages, has_message, set_message, set_messages};

pub struct AppState {
    channels: DashMap<u64, Channel>,
    messages: DashMap<u64, Message>,
    guilds: DashMap<u64, Guild>,
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
        guilds: DashMap::new(),
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

            .service(set_guilds)
            .service(get_guilds)
            .service(set_guild)
            .service(get_guild)
            .service(has_guild)
            .service(delete_guild)
    })
    .bind_openssl("127.0.0.1:9493", builder)?
    .run()
    .await
}
