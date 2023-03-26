use std::num::ParseIntError;

use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name, Class, Predicate};

use crate::error::{Error, Result};
use crate::models::AnimeUser;

use super::{AnimeUserEntry, Anime};

pub(crate) fn parse_anime_list(user_id: u32, body: String) -> Result<Vec<AnimeUserEntry>>{
    let document = Document::from(body.as_str());
    //let document = Document::from(include_str!("../../anime.html"));
    let anime_list = parse_anime_list_from_doc(user_id, &document)?;
    Ok(anime_list)
}

fn parse_anime_list_from_doc(user_id: u32, document: &Document) -> Result<Vec<AnimeUserEntry>> {
    let main_table = get_main_table(&document)?;
    let rows = get_rows(main_table)?;
    let user = parse_user_name(&document)?;
    let parsed_rows = rows.into_iter().map(|row| parse_row(user_id, user.as_str(), row)).collect::<Result<Vec<AnimeUserEntry>>>()?;

    Ok(parsed_rows)
}

fn get_main_table(doc: &Document) -> Result<Node> {
    let mut tables = doc.find(Attr("id", "main").descendant(Name("table")));
    let first_table = tables.next();
    match first_table {
        Some(table) => Ok(table),
        None => {
            if check_no_access_permissions(doc) {
                return Err(Error::ProxerAccessError());
            } else {
                return Err(Error::MainTableParseError())
            }
        }
    }
}

const NO_PERMISSION_TEXT: &str = "Du hast keine Berechtigung um diese Seite zu betreten.";
fn check_no_access_permissions(doc: &Document) -> bool {
    let heading_text = doc.find(Attr("id", "main").descendant(Name("h3")))
        .next()
        .map(|h| h.inner_html());
    heading_text.unwrap_or_else(|| String::new()) == NO_PERMISSION_TEXT
}

fn get_rows(node: Node) -> Result<Vec<Node>> {
    let rows = node.find(Name("tr")).filter(|row| row.find(Name("th")).count()==0).collect::<Vec<_>>();
    return if rows.is_empty() {
        Err(Error::AnimeRowParseError())
    } else {
        Ok(rows)
    }
}

fn parse_user_name(doc: &Document) -> Result<String> {
    let user = doc.find(Attr("id", "main").child(Attr("id", "simple-navi"))).next()
        .map(|tabs| tabs.first_child()).flatten() // li: Tab Overview
        .map(|tab| tab.first_child()).flatten() // a: Link + Text
        .map(|tab| tab.inner_html())
        .map(|text| {
            println!("{}", text);
            let regex = Regex::new(r"Profil: (.+)").unwrap();
            let cap = regex.captures_iter(&text).next();
            cap.map(|cap| cap[1].to_string())
        }).flatten();
    match user {
        Some(user) => Ok(user),
        None => Err(Error::UserNameParseError())
    }
}

fn parse_row(user_id: u32, user: &str, node: Node) -> Result<AnimeUserEntry> {
    let title = parse_title(node)?;
    let title_id = parse_title_id(node)?;
    let (progress, episode_count) = parse_progress(node)?;
    Ok( AnimeUserEntry::new(Anime::new(title_id, title, None, episode_count), AnimeUser::new(user_id, user.to_string()), progress) )
}

fn parse_title(node: Node) -> Result<String> {
    let title_node = node.find(Name("td").descendant(Name("a").and(Class("tip"))))
        .next()
        .ok_or_else(|| Error::AnimeRowFieldParseError("title".to_string()))?;
    Ok(title_node.inner_html())
}

fn parse_title_id(node: Node) -> Result<u32> {
    let title_id = node.find(Name("td").descendant(Name("a").and(Class("tip"))))
        .next()
        .and_then(|a| {
            a.attr("href")
        })
        .and_then(|href| {
            let regex = Regex::new(r"\d+").unwrap();
            regex.find(href)
        })
        .and_then(|id_match| {
            id_match.as_str().parse::<u32>().ok()
        })
        .ok_or_else(|| Error::AnimeRowFieldParseError("title id".to_string()))?;
    Ok(title_id)
}

fn parse_progress(node: Node) -> Result<(u32, u32)> {
    let progress = node.find(Name("td").descendant(Name("span").and(Class("state"))))
        .next()
        .and_then(|span| {
            let inner_html = span.inner_html();
            let values = inner_html.split("/")
                .map(|s| s.trim())
                .map(|s| s.parse::<u32>()).collect::<std::result::Result<Vec<u32>, ParseIntError>>()
                .ok();
            match values {
                Some(values) => if values.len()==2 {
                        Some((values[0], values[1]))
                    } else {
                        None
                    },
                None => None
            }
        })
        .ok_or_else(|| Error::AnimeRowFieldParseError("title".to_string()))?;
    Ok(progress)
}

#[cfg(test)]
mod test {
    use select::document::Document;


    use super::parse_anime_list_from_doc;


    #[test]
    fn read_info() {
        let document = Document::from(include_str!("../../anime.html"));
        let anime_list = parse_anime_list_from_doc(0, &document).unwrap();
        anime_list.iter().take(3).for_each(|a| println!("{}", a))
    }
}