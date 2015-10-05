use kuchiki::Html;
use selectors::Element;

use data::{Id, Rating, Item};
use helpers::{ElementDataRef, QuerySelector};

fn get_item_id(elem: &ElementDataRef) -> Id {
    static ID_PREFIX: &'static str = "item_";
    let id = elem.get_id().unwrap();
    let id_str = id.as_slice();
    assert!(id_str.starts_with(ID_PREFIX));
    id_str[ID_PREFIX.len()..].parse().unwrap()
}

fn get_item_title(elem: &ElementDataRef) -> String {
    let title_node = elem.query_selector("h3>*:last-child").unwrap();
    assert!(title_node.name == qualname!(html, small) ||
            title_node.name == qualname!(html, a));
    title_node.text_contents()
}

fn get_item_rating(elem: &ElementDataRef) -> Option<Rating> {
    static STARS_PREFIX: &'static str = "sstars";
    elem.query_selector(".starsinfo").map(|e| {
        let mut result = None;
        e.each_class(|class| {
            let class_str = class.as_slice();
            if class_str.starts_with(STARS_PREFIX) {
                let rating = class_str[STARS_PREFIX.len()..].parse().unwrap();
                assert!(rating >= 1 && rating <= 10);
                result = Some(rating);
            }
        });
        result.unwrap()
    })
}

fn get_item_tags(elem: &ElementDataRef) -> Vec<String> {
    static TAGS_PREFIX: &'static str = "标签: ";
    if let Some(tags_elem) = elem.query_selector(".collectInfo>.tip") {
        let all_text = tags_elem.text_contents();
        assert!(all_text.starts_with(TAGS_PREFIX));
        all_text[TAGS_PREFIX.len()..].split(" ")
            .filter_map(|s| {
                if !s.is_empty() {
                    Some(s.to_string())
                } else {
                    None
                }
            }).collect()
    } else {
        vec![]
    }
}

fn generate_item_from_node(elem: &ElementDataRef) -> Item {
    Item {
        id: get_item_id(elem),
        title: get_item_title(elem),
        rating: get_item_rating(elem),
        tags: get_item_tags(elem)
    }
}

pub fn get_all_items(html: Html) -> Vec<Item> {
    html.parse().select("#browserItemList>li").unwrap()
        .map(|elem| generate_item_from_node(&elem)).collect()
}
