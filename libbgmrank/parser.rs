use crate::data::{Id, Item, Rating};
use crate::helpers::{ElementDataRef, QuerySelector};
use html5ever::{expanded_name, local_name, namespace_url, ns};
use kuchiki::NodeRef;

fn get_item_id(elem: &ElementDataRef) -> Id {
    static ID_PREFIX: &'static str = "item_";
    let attrs = elem.attributes.borrow();
    let id = attrs.get(local_name!("id")).unwrap();
    let (prefix, id_str) = id.split_at(ID_PREFIX.len());
    assert!(prefix == ID_PREFIX);
    id_str.parse().unwrap()
}

fn get_item_title(elem: &ElementDataRef) -> String {
    let title_node = elem.query_selector("h3>*:last-child").unwrap();
    assert!(
        title_node.name.expanded() == expanded_name!(html "small")
            || title_node.name.expanded() == expanded_name!(html "a")
    );
    title_node.text_contents()
}

fn get_item_rating(elem: &ElementDataRef) -> Option<Rating> {
    static STARS_PREFIX: &'static str = "stars";
    let elem = elem.query_selector(".starlight")?;
    let attrs = elem.attributes.borrow();
    let classes = attrs.get(local_name!("class")).unwrap();
    let result = classes
        .split_whitespace()
        .find_map(|class| {
            if !class.starts_with(STARS_PREFIX) {
                return None;
            }
            let rating = class[STARS_PREFIX.len()..].parse().unwrap();
            assert!(rating >= 1 && rating <= 10);
            Some(rating)
        })
        .unwrap();
    Some(result)
}

fn get_item_tags(elem: &ElementDataRef) -> Vec<String> {
    static TAGS_PREFIX: &'static str = "标签: ";
    if let Some(tags_elem) = elem.query_selector(".collectInfo>.tip") {
        let all_text = tags_elem.text_contents();
        let tag_text = all_text.trim();
        assert!(tag_text.starts_with(TAGS_PREFIX));
        tag_text[TAGS_PREFIX.len()..]
            .split(" ")
            .filter_map(|s| {
                if !s.is_empty() {
                    Some(s.to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn generate_item_from_node(elem: &ElementDataRef) -> Item {
    Item {
        id: get_item_id(elem),
        title: get_item_title(elem),
        rating: get_item_rating(elem),
        tags: get_item_tags(elem),
    }
}

pub fn get_all_items(html: NodeRef) -> Vec<Item> {
    html.select("#browserItemList>li")
        .unwrap()
        .map(|elem| generate_item_from_node(&elem))
        .collect()
}
