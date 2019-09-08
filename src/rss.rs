use crate::mangarockparser::{ChapterInfo, MangaInfo};
use chrono::{DateTime, Utc};

pub fn rss_for(infos: &Vec<MangaInfo>, now: DateTime<Utc>) -> String {
    format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Manga rock</title>
    <description>Ceci un flux RSS reconstruit pour manga rock</description>
    <lastBuildDate>{lastBuildDate}</lastBuildDate>
    <link>https://mangarock.com/g</link>
{items}
  </channel>
</rss>
    "#,
        lastBuildDate = now.to_rfc2822(),
        items = infos
            .iter()
            .flat_map(|info| last_n_elements(&info.chapters, 10)
                .iter()
                .map(|chapter| build_item(info, chapter))
                .collect::<Vec<String>>())
            .collect::<Vec<String>>()
            .join("\n")
    )
    .trim()
    .to_owned()
}

fn last_n_elements<T>(v: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    let last_index = v.len();

    if last_index < n {
        v.to_vec()
    } else {
        v[(last_index - n)..].to_vec()
    }
}

fn build_item(manga_info: &MangaInfo, item: &ChapterInfo) -> String {
    format!(
        r#"    <item>
      <title>{title}</title>
      <description>{description}</description>
      <pubDate>{pubDate}</pubDate>
      <link>{link}</link>
    </item>"#,
        title = format!("{} - {}", manga_info.name, item.name),
        description = item.name,
        pubDate = item.updated_datetime().to_rfc2822(),
        link = format!(
            "https://mangarock.com/manga/{}/chapter/{}",
            manga_info.oid, item.oid
        )
    )
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mangarockparser::{ChapterOid, MangaInfo, MangaOid, Timestamp};
    use chrono::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_get_last_items_if_vec_bigger_than_n() {
        let result = last_n_elements(&vec![2, 3, 4, 5, 6], 3);
        assert_eq!(result, vec![4, 5, 6]);
    }
    #[test]
    fn it_should_get_whole_list_of_items_if_vec_smaller_than_n() {
        let result = last_n_elements(&vec![2, 3, 4, 5, 6], 10);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn it_should_generate_expected_rss() {
        let infos: Vec<MangaInfo> = vec![
            MangaInfo::new(
                MangaOid::new("mrs-serie-288364"),
                "3rei!!".to_owned(),
                Timestamp::new(1553693344),
                vec![
                    ChapterInfo::new(
                        ChapterOid::new("mrs-chapter-35593"),
                        "chapter A".to_owned(),
                        Timestamp::new(1553691344),
                    ),
                    ChapterInfo::new(
                        ChapterOid::new("mrs-chapter-35594"),
                        "chapter B".to_owned(),
                        Timestamp::new(1553693344),
                    ),
                ],
            ),
            MangaInfo::new(
                MangaOid::new("mrs-serie-123456"),
                "My awesome manga".to_owned(),
                Timestamp::new(1553683344),
                vec![
                    ChapterInfo::new(
                        ChapterOid::new("mrs-chapter-45593"),
                        "chapter 1".to_owned(),
                        Timestamp::new(1553661344),
                    ),
                    ChapterInfo::new(
                        ChapterOid::new("mrs-chapter-55594"),
                        "chapter 2".to_owned(),
                        Timestamp::new(1553683344),
                    ),
                ],
            ),
        ];

        let now = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11);

        let result = rss_for(&infos, now);

        assert_eq!(
            result,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Manga rock</title>
    <description>Ceci un flux RSS reconstruit pour manga rock</description>
    <lastBuildDate>Tue,  8 Jul 2014 09:10:11 +0000</lastBuildDate>
    <link>https://mangarock.com/g</link>
    <item>
      <title>3rei!! - chapter A</title>
      <description>chapter A</description>
      <pubDate>Wed, 27 Mar 2019 12:55:44 +0000</pubDate>
      <link>https://mangarock.com/manga/mrs-serie-288364/chapter/mrs-chapter-35593</link>
    </item>
    <item>
      <title>3rei!! - chapter B</title>
      <description>chapter B</description>
      <pubDate>Wed, 27 Mar 2019 13:29:04 +0000</pubDate>
      <link>https://mangarock.com/manga/mrs-serie-288364/chapter/mrs-chapter-35594</link>
    </item>
    <item>
      <title>My awesome manga - chapter 1</title>
      <description>chapter 1</description>
      <pubDate>Wed, 27 Mar 2019 04:35:44 +0000</pubDate>
      <link>https://mangarock.com/manga/mrs-serie-123456/chapter/mrs-chapter-45593</link>
    </item>
    <item>
      <title>My awesome manga - chapter 2</title>
      <description>chapter 2</description>
      <pubDate>Wed, 27 Mar 2019 10:42:24 +0000</pubDate>
      <link>https://mangarock.com/manga/mrs-serie-123456/chapter/mrs-chapter-55594</link>
    </item>
  </channel>
</rss>"#
        )
    }
}
