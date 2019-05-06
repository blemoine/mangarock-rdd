use chrono::offset::TimeZone;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ChapterOid(String);

impl ChapterOid {
    pub fn new(oid: &str) -> ChapterOid {
        ChapterOid(oid.to_owned())
    }
}

impl Display for ChapterOid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MangaOid(String);

impl MangaOid {
    pub fn new(oid: &str) -> MangaOid {
        MangaOid(oid.to_owned())
    }
}

impl Display for MangaOid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn new(ts: i64) -> Timestamp {
        Timestamp(ts)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChapterInfo {
    pub oid: ChapterOid,
    pub name: String,
    #[serde(rename = "updatedAt")]
    updated_at: Timestamp,
}

impl ChapterInfo {
    pub fn updated_datetime(&self) -> DateTime<Utc> {
        Utc.timestamp(self.updated_at.0, 0)
    }

    pub fn new(oid: ChapterOid, name: String, updated_at: Timestamp) -> ChapterInfo {
        ChapterInfo {
            oid,
            name,
            updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MangaInfo {
    pub oid: MangaOid,
    pub name: String,
    last_update: Timestamp,
    pub chapters: Vec<ChapterInfo>,
}

impl MangaInfo {
    pub fn new(
        oid: MangaOid,
        name: String,
        last_update: Timestamp,
        chapters: Vec<ChapterInfo>,
    ) -> MangaInfo {
        MangaInfo {
            oid,
            name,
            last_update,
            chapters,
        }
    }
}

// Look at https://stackoverflow.com/questions/54657873/is-there-a-way-to-omit-wrapper-root-objects-when-deserializing-objects-with-serd is generalisation is needed

#[derive(Serialize, Deserialize, Debug)]
struct MangaInfoWrapper {
    data: MangaInfo,
}

pub fn parse_manga_info(data: &str) -> Result<MangaInfo, Error> {
    let r = serde_json::from_str::<MangaInfoWrapper>(data)?;

    Ok(r.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_should_work() {
        let data = r#"
       {
	"code": 0,
	"data": {
		"mid": 553712,
		"oid": "mrs-serie-35593",
		"name": "Boruto: Naruto Next Generations",
		"author": "Ukyo Kodachi",
		"rank": 199,
		"msid": 71,
		"completed": false,
		"last_update": 1555711356,
		"removed": false,
		"direction": 1,
		"total_chapters": 35,
		"description": "Naruto was a young shinobi with an incorrigible knack for mischief. He achieved his dream to become the greatest ninja in the village and his face sits atop the Hokage monument. But this is not his story... A new generation of ninja are ready to take the stage, led by Naruto's own son, Boruto!",
		"categories": [1, 2, 3, 4, 5, 8, 27, 41],
		"chapters": [{
			"cid": 28834048,
			"oid": "mrs-chapter-100410084",
			"order": 31,
			"name": "Vol.TBD Chapter 31: Monster...!",
			"updatedAt": 1548474140
		}, {
			"cid": 28983642,
			"oid": "mrs-chapter-100426942",
			"order": 32,
			"name": "Vol.TBD Chapter 32: A Sense of Duty",
			"updatedAt": 1550854922
		}, {
			"cid": 29112284,
			"oid": "mrs-chapter-200002666",
			"order": 33,
			"name": "Vol.TBD Chapter 33: Breaking The Limit",
			"updatedAt": 1553223666
		}, {
			"cid": 29227830,
			"oid": "mrs-chapter-200023474",
			"order": 34,
			"name": "Vol.TBD Chapter 34: Training!!",
			"updatedAt": 1555711346
		}],
		"thumbnail": "https://f01.mrcdn.info/file/mrportal/i/5/8/3/G3.6PwgFb_B.jpg",
		"cover": "https://f01.mrcdn.info/file/mrportal/h/c/3/0/J_.h_1FHZfW.jpg",
		"artworks": ["https://f01.mrcdn.info/file/mrportal/i/5/8/2/46.jrpZSy5Z.jpg", "https://f01.mrcdn.info/file/mrportal/i/5/8/2/4k.dTA7v5Tr.jpg", "https://f01.mrcdn.info/file/mrportal/j/3/7/4/b-.1uiOTSCI.jpg"],
		"alias": ["Boruto: Naruto Next Generation", "BORUTO-NARUTO NEXT GENERATIONS-", "Boruto"],
		"characters": [{
			"oid": "mrs-character-311684",
			"name": "Mitsuki",
			"thumbnail": "https://f01.mrcdn.info/file/mrportal/h/6/r/5/ir.gzDRU8YT.png"
		}, {
			"oid": "mrs-character-311685",
			"name": "Sarada Uchiha",
			"thumbnail": "https://f01.mrcdn.info/file/mrportal/h/6/r/3/s9.2-Ug0Zfx.png"
		}],
		"authors": [{
			"oid": "mrs-author-306911",
			"name": "Ukyo Kodachi",
			"thumbnail": "https://f01.mrcdn.info/file/mrportal/i/5/7/g/ej.vP9TUgn.jpg",
			"role": "story"
		}, {
			"oid": "mrs-author-311666",
			"name": "Mikio Ikemoto",
			"thumbnail": "",
			"role": "art"
		}],
		"rich_categories": [{
			"oid": "mrs-genre-304068",
			"name": "Action"
		}, {
			"oid": "mrs-genre-304069",
			"name": "Comedy"
		}],
		"extra": {
			"English Publisher": "Viz",
			"Original Publisher": "Shueisha ",
			"Published": "May 9, 2016 ",
			"Serialization": "Shuukan Shounen Jump"
		},
		"mrs_series": null
	}
}"#;

        let v: MangaInfo = parse_manga_info(data).unwrap();

        assert_eq!(v.oid, MangaOid("mrs-serie-35593".to_owned()));
        assert_eq!(v.last_update, Timestamp(1555711356));
        assert_eq!(v.chapters.len(), 4);
    }
}
