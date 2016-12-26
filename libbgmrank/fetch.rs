use kuchiki;
use kuchiki::traits::ParserExt;

use parser;
use data::{ToStaticStr, Category, State, Item};

const ITEMS_PER_PAGE: usize = 24;

pub fn get_items<C>(username: &str, category: Category, state: State, callback: C) -> Vec<Item>
    where C: Fn(usize)
{
    let category_str = category.to_static_str();
    let state_str = state.to_static_str();
    let mut result = vec![];
    for page in 1.. {
        callback(page);
        let url = format!("https://bgm.tv/{}/list/{}/{}?page={}",
                          category_str,
                          username,
                          state_str,
                          page);
        let doc = kuchiki::parse_html().from_http(&url).unwrap();
        let items = parser::get_all_items(doc);
        let count = items.len();
        result.extend(items);
        if count < ITEMS_PER_PAGE {
            break;
        }
    }
    result
}
