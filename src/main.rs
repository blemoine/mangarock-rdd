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

fn index(_req: &HttpRequest) -> Result<String, MangaError>  {
    let oids: Vec<MangaOid> = vec![
        "mrs-serie-288364", // 3rei,
        "mrs-serie-26314",  // berserk
        "mrs-serie-303939", //the promised neverland
        "mrs-serie-295440", //shingeki no kyojin
    ]
        .iter()
        .map(|s| MangaOid::new(s))
        .collect();

    let infos = oids.iter().map(|oid| info_on(oid)).collect::<Result<Vec<MangaInfo>, MangaError>>()?;

    Ok(rss_for(&infos, Utc::now()))
}

fn main() {
    server::new(|| App::new().resource("/feed.rss", |r| r.f(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
}
