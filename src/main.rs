mod mangarockclient;
mod mangarockparser;
mod rss;

use mangarockparser::MangaInfo;
use mangarockclient::info_on;
use mangarockparser::MangaOid;
use actix_web::{HttpRequest, server, App};
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
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());

    server::new(|| App::new().resource("/feed.rss", |r| r.f(index)))
        .bind(format!("127.0.0.1:{}", port))
        .unwrap()
        .run()
}
