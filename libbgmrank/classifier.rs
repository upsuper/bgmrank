use crate::data::Item;
use std::collections::HashMap;

fn normalize_tag(tag: &str) -> String {
    tag.to_lowercase()
}

pub fn classify_by_tags<'a>(items: &'a [Item]) -> HashMap<String, Vec<&'a Item>> {
    let mut tags_map = HashMap::new();
    let mut items_map = HashMap::new();
    for item in items {
        for tag in item.tags.iter() {
            let normalized = normalize_tag(tag);
            let tag_item = tags_map
                .entry(normalized.clone())
                .or_insert_with(|| HashMap::new());
            *tag_item.entry(tag).or_insert(0usize) += 1;
            items_map
                .entry(normalized)
                .or_insert_with(|| vec![])
                .push(item);
        }
    }
    let mut result = HashMap::with_capacity(tags_map.len());
    for (tag, display_tags) in tags_map.into_iter() {
        let display_tag = display_tags
            .into_iter()
            .fold(
                (tag.clone(), 0),
                |(cur_tag, cur_num), (new_tag, new_num)| {
                    if new_num > cur_num {
                        (new_tag.clone(), new_num)
                    } else {
                        (cur_tag, cur_num)
                    }
                },
            )
            .0;
        result.insert(display_tag, items_map.remove(&tag).unwrap());
    }
    result
}
