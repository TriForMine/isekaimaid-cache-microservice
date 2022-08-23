use actix_web::{get, Error, HttpResponse};
use std::io::Cursor;
use memory_stats::memory_stats;

#[get("/stats")]
pub async fn get_stats() -> Result<HttpResponse, Error> {
    let mut buff = Cursor::new(Vec::new());

    if let Some(usage) = memory_stats() {
        cbor4ii::serde::to_writer(&mut buff, &usage.physical_mem).unwrap();
    } else {
        println!("Couldn't get the current memory usage :(");
        cbor4ii::serde::to_writer(&mut buff, &-1).unwrap();
    }

    let res = buff.get_ref();

    Ok(HttpResponse::Ok().body(res.clone()))
}
