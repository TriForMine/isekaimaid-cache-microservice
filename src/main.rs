mod channels;
mod guilds;
mod members;
mod messages;
mod types;
mod users;
mod permissions;
mod roles;

use crate::channels::{
    delete_channel, get_channel, get_channels, get_channels_size, has_channel, set_channel,
    set_channels,
};
use crate::guilds::{
    delete_guild, get_guild, get_guilds, get_guilds_members_size, get_guilds_size, has_guild,
    set_guild, set_guilds,
};
use crate::members::{
    delete_member, get_member, get_members, get_members_size, has_member, set_member, set_members,
};
use crate::messages::{
    delete_message, get_message, get_messages, get_messages_size, has_message, set_message,
    set_messages,
};
use crate::types::{Channel, Guild, Member, Message, Role, User};
use crate::users::{
    delete_user, get_user, get_users, get_users_size, has_user, set_user, set_users,
};
use actix_web::{web, App, HttpServer};
use dashmap::DashMap;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::runtime::{Handle};
use crate::roles::{delete_role, get_role, get_roles, get_roles_size, has_role, set_role, set_roles};

pub struct AppState {
    channels: DashMap<u64, Channel>,
    messages: DashMap<u64, Message>,
    guilds: DashMap<u64, Guild>,
    users: DashMap<u64, User>,
    members: DashMap<String, Member>,
    roles: DashMap<u64, Role>,
}

pub fn spawn(data: Arc<AppState>, handle: Handle) {
    println!("[CACHE] Spawning threads");

    handle.spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300));

        loop {
            interval.tick().await;

            println!("[CACHE] Clearing old messages");

            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            for message in data.messages.iter() {
                let id = message.key();
                let message = message.value();
                let timestamp = message.timestamp.as_ref().unwrap();

                if timestamp + 1000 * 60 * 15 < current_time {
                    println!("[CACHE] Deleting old message: {}", id,);
                    data.messages.remove(&id);
                }
            }
        }
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let state = Arc::new(AppState {
        channels: DashMap::new(),
        messages: DashMap::new(),
        guilds: DashMap::new(),
        users: DashMap::new(),
        members: DashMap::new(),
        roles: DashMap::new(),
    });

    /*
    let rt = Runtime::new().unwrap();
    let handle = rt.handle().clone();
    spawn( state.clone(), handle);*/

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
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
            .service(set_roles)
            .service(get_roles)
            .service(set_role)
            .service(get_role)
            .service(has_role)
            .service(delete_role)
            .service(get_roles_size)
    })
    .bind_openssl("127.0.0.1:9493", builder)?
    .run()
    .await
}
