use crate::data::{Category, Item, State};
use crate::parser;
use html5ever::tendril::stream::TendrilSink;
use kuchiki::NodeRef;
use reqwest::Client;
use std::error::Error;

const ITEMS_PER_PAGE: usize = 24;

fn fetch_page(client: &Client, url: &str) -> Result<NodeRef, Box<dyn Error>> {
    client.get(url).send()?;
    let mut resp = client.get(url).send()?;
    Ok(kuchiki::parse_html().from_utf8().read_from(&mut resp)?)
}

pub fn get_items(
    username: &str,
    category: Category,
    state: State,
    callback: impl Fn(usize),
) -> Vec<Item> {
    let category_str: &str = category.into();
    let state_str: &str = state.into();
    let client = Client::new();
    let mut result = vec![];
    for page in 1.. {
        callback(page);
        let url = format!(
            "https://bgm.tv/{}/list/{}/{}?page={}",
            category_str, username, state_str, page
        );
        let doc = fetch_page(&client, &url).unwrap();
        let items = parser::get_all_items(doc);
        let count = items.len();
        result.extend(items);
        if count < ITEMS_PER_PAGE {
            break;
        }
    }
    result
}
