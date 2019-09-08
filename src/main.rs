mod mangarockclient;
mod mangarockparser;
mod rss;

use mangarockclient::info_on;
use mangarockparser::MangaOid;
use actix_web::{HttpRequest, server, App, HttpResponse};
use chrono::Utc;
use rss::rss_for;
use mangarockclient::MangaError;

impl actix_web::error::ResponseError for MangaError {}

fn index(_req: &HttpRequest) -> Result<String, MangaError> {
    let oids: Vec<MangaOid> = vec![
        "mrs-serie-288364", // 3rei,
        "mrs-serie-26314",  // berserk
        "mrs-serie-303939", //the promised neverland
        "mrs-serie-295440", //shingeki no kyojin
        "mrs-serie-132938", //Kimetsu no yaiba
    ]
        .iter()
        .map(|s| MangaOid::new(s))
        .collect();


    let infos = oids.iter().filter_map(|oid| {
        match info_on(oid) {
            Ok(info) => Some(info),
            Err(err) => {
                println!("Got an error for manga {}: {}", oid, err);
                None
            }
        }
    }).collect();

    Ok(rss_for(&infos, Utc::now()))
}

fn main() {
    let http_bind = std::env::var("HTTP_BIND").unwrap_or("0.0.0.0".to_string());
    let http_port = std::env::var("PORT").unwrap_or("8088".to_string());
    let bind_url = format!("{}:{}", http_bind, http_port);
    println!("Binding to {}", bind_url);

    server::new(|| App::new()
        .resource("/", |r| r.get().f(|_| HttpResponse::Ok()))
        .resource("/feed.rss", |r| r.f(index)))
        .bind(bind_url)
        .unwrap()
        .run()
}
