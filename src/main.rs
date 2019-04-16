extern crate actix_web;

use actix_web::{server, App, HttpRequest};
use chrono::{DateTime, Utc};
use std::prelude::v1::*;

struct Item {
    title: String,
    description: String,
    pub_date: DateTime<Utc>,
    link: String,
}

fn build_item(item: &Item) -> String {
    format!(
        r#"
 <item>
    <title>{title}</title>
    <description>{description}</description>
    <pubDate>{pubDate}</pubDate>
    <link>{link}</link>
</item>
"#,
        title = item.title,
        description = item.description,
        pubDate = item.pub_date.to_rfc2822(),
        link = item.link
    )
    .trim()
    .to_owned()
}

fn build_items(items: &[Item]) -> String {
    items
        .iter()
        .map(build_item)
        .collect::<Vec<String>>()
        .concat()
}

fn index(_req: &HttpRequest) -> String {
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
        lastBuildDate = "Sat, 07 Sep 2002 00:00:01 GMT",
        items = build_items(&vec![])
    )
    .trim()
    .to_owned()
}

fn main() {
    server::new(|| App::new().resource("/feed.rss", |r| r.f(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
}

#[cfg(test)]
mod tests {
    use crate::{build_item, build_items, Item};
    use chrono::prelude::*;
    use chrono::Utc;
    use pretty_assertions::assert_eq;

    #[test]
    fn build_item_should_work() {
        let item = Item {
            title: "title Test".to_owned(),
            description: "a description".to_owned(),
            pub_date: Utc.ymd(2018, 1, 26).and_hms_micro(18, 30, 9, 453_829),
            link: "http://google.fr".to_owned(),
        };
        assert_eq!(
            build_item(&item),
            r#"
 <item>
    <title>title Test</title>
    <description>a description</description>
    <pubDate>Fri, 26 Jan 2018 18:30:09 +0000</pubDate>
    <link>http://google.fr</link>
</item>
"#
            .trim()
        );
    }

    #[test]
    fn build_items_should_work() {
        let item1 = Item {
            title: "title Test".to_owned(),
            description: "a description".to_owned(),
            pub_date: Utc.ymd(2018, 1, 26).and_hms_micro(18, 30, 9, 453_829),
            link: "http://google.fr".to_owned(),
        };
        let item2 = Item {
            title: "another title Test".to_owned(),
            description: "another description".to_owned(),
            pub_date: Utc.ymd(2018, 1, 19).and_hms_micro(16, 23, 29, 453_829),
            link: "http://google.com".to_owned(),
        };
        assert_eq!(
            build_items(&vec![item1, item2]),
            r#"
<item>
    <title>title Test</title>
    <description>a description</description>
    <pubDate>Fri, 26 Jan 2018 18:30:09 +0000</pubDate>
    <link>http://google.fr</link>
</item><item>
    <title>another title Test</title>
    <description>another description</description>
    <pubDate>Fri, 19 Jan 2018 16:23:29 +0000</pubDate>
    <link>http://google.com</link>
</item>
"#
            .trim()
        );
    }
}
