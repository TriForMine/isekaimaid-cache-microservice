mod channels;
mod types;
mod messages;
mod guilds;
mod users;
mod members;

use crate::channels::{delete_channel, get_channel, get_channels, get_channels_size, has_channel, set_channel, set_channels};
use crate::types::{Channel, Guild, Member, Message, User};
use actix_web::{web, App, HttpServer};
use dashmap::DashMap;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::guilds::{delete_guild, get_guild, get_guilds, get_guilds_members_size, get_guilds_size, has_guild, set_guild, set_guilds};
use crate::members::{delete_member, get_member, get_members, get_members_size, has_member, set_member, set_members};
use crate::messages::{delete_message, get_message, get_messages, get_messages_size, has_message, set_message, set_messages};
use crate::users::{delete_user, get_user, get_users, get_users_size, has_user, set_user, set_users};

pub struct AppState {
    channels: DashMap<u64, Channel>,
    messages: DashMap<u64, Message>,
    guilds: DashMap<u64, Guild>,
    users: DashMap<u64, User>,
    members: DashMap<String, Member>,
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
        users: DashMap::new(),
        members: DashMap::new(),
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
            .service(get_channels_size)

            .service(set_messages)
            .service(get_messages)
            .service(set_message)
            .service(get_message)
            .service(has_message)
            .service(delete_message)
            .service(get_messages_size)

            .service(set_guilds)
            .service(get_guilds)
            .service(set_guild)
            .service(get_guild)
            .service(has_guild)
            .service(delete_guild)
            .service(get_guilds_size)
            .service(get_guilds_members_size)

            .service(set_users)
            .service(get_users)
            .service(set_user)
            .service(get_user)
            .service(has_user)
            .service(delete_user)
            .service(get_users_size)

            .service(set_members)
            .service(get_members)
            .service(set_member)
            .service(get_member)
            .service(has_member)
            .service(delete_member)
            .service(get_members_size)
    })
    .bind_openssl("127.0.0.1:9493", builder)?
    .run()
    .await
}
