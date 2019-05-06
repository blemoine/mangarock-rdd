use crate::mangarockparser::{parse_manga_info, MangaInfo, MangaOid};

use std::fmt::{Display, Formatter};

const BASE_URL: &str = "https://api.mangarockhd.com/query/web401";


#[derive(Debug)]
pub enum MangaError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl From<reqwest::Error> for MangaError {
    fn from(err: reqwest::Error) -> MangaError {
        MangaError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for MangaError {
    fn from(err: serde_json::Error) -> MangaError {
        MangaError::SerdeError(err)
    }
}

impl Display for MangaError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        match self {
            MangaError::ReqwestError(err) => err.fmt(f),
            MangaError::SerdeError(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for MangaError {}


pub fn info_on(oid: &MangaOid) -> Result<MangaInfo, MangaError> {
    let url = format!("{base_url}/info?oid={oid}", base_url = BASE_URL, oid = oid);

    let response: String = reqwest::get(url.as_str())?.error_for_status()?.text()?;
    let result = parse_manga_info(response.as_str())?;

    Ok(result)
}
