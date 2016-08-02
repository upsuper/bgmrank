use selectors::Element;
use kuchiki::NodeRef;

use data::{Id, Rating, Item};
use helpers::{ElementDataRef, QuerySelector};

fn get_item_id(elem: &ElementDataRef) -> Id {
    static ID_PREFIX: &'static str = "item_";
    let id = elem.get_id().unwrap();
    let (prefix, id_str) = id.split_at(ID_PREFIX.len());
    assert!(prefix == ID_PREFIX);
    id_str.parse().unwrap()
}

fn get_item_title(elem: &ElementDataRef) -> String {
    let title_node = elem.query_selector("h3>*:last-child").unwrap();
    assert!(title_node.name == qualname!(html, "small") ||
            title_node.name == qualname!(html, "a"));
    title_node.text_contents()
}

fn get_item_rating(elem: &ElementDataRef) -> Option<Rating> {
    static STARS_PREFIX: &'static str = "sstars";
    elem.query_selector(".starsinfo").map(|e| {
        let mut result = None;
        e.each_class(|class| {
            let (prefix, class_str) = class.split_at(STARS_PREFIX.len());
            if prefix == STARS_PREFIX {
                let rating = class_str.parse().unwrap();
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
        let tag_text = all_text.trim();
        assert!(tag_text.starts_with(TAGS_PREFIX));
        tag_text[TAGS_PREFIX.len()..].split(" ")
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

pub fn get_all_items(html: NodeRef) -> Vec<Item> {
    html.select("#browserItemList>li").unwrap()
        .map(|elem| generate_item_from_node(&elem)).collect()
}
