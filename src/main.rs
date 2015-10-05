extern crate enum_set;
extern crate getopts;
extern crate hyper;
extern crate kuchiki;
extern crate selectors;
#[macro_use] extern crate string_cache;

mod classifier;
mod data;
mod init;
mod helpers;
mod parser;
mod stats;

use std::cmp::PartialOrd;
use hyper::client::Client;
use kuchiki::Html;

use data::{ToStaticStr, Category, State, Item, MAX_RATING};

const ITEMS_PER_PAGE: usize = 24;

fn get_items(username: &str, category: Category, state: State) -> Vec<Item> {
    let category_str = category.to_static_str();
    let state_str = state.to_static_str();
    let client = Client::new();
    let mut result = vec![];
    println!("fetching {}: {}/{}", username, category_str, state_str);
    for page in 1.. {
        println!("  fetching page {}...", page);
        let url = format!("https://bgm.tv/{}/list/{}/{}?page={}",
                          category_str, username, state_str, page);
        let mut res = client.get(&url).send().unwrap();
        if res.status != hyper::Ok {
            println!("Failed to fetch {}: {}", url, res.status);
            std::process::exit(1);
        }
        let html = Html::from_stream(&mut res).unwrap();
        let items = parser::get_all_items(html);
        let count = items.len();
        result.extend(items);
        if count < ITEMS_PER_PAGE {
            break;
        }
    }
    result
}

fn get_all_items(args: &init::Args) -> Vec<Item> {
    let mut result = vec![];
    for category in args.categories.iter() {
        for state in args.states.iter() {
            result.extend(get_items(&args.username, category, state));
        }
    }
    println!("");
    result
}

struct TagStats {
    tag: String,
    avg: f32,
    stdev: f32,
    rated: usize,
    total: usize
}

fn generate_tag_stats(all_items: &Vec<Item>) -> Vec<TagStats> {
    let mut result: Vec<TagStats> = classifier::classify_by_tags(all_items)
        .into_iter().filter_map(|(tag, items)| {
            let total = items.len();
            let hist = stats::get_histogram(items.into_iter());
            let (avg, stdev) = hist.get_avg_and_stdev();
            if avg.is_nan() || stdev.is_nan() {
                return None;
            }
            Some(TagStats {
                tag: tag,
                avg: avg,
                stdev: stdev,
                rated: hist.get_all_rated(),
                total: total
            })
        }).collect();
    result.sort_by(|l, r| {
        (l.avg, -l.stdev).partial_cmp(&(r.avg, -r.stdev))
            // It should be safe to unwrap here because we should have
            // filtered out all NaNs in the loop above.
            .unwrap().reverse()
    });
    result
}

fn generate_bar(width: usize) -> String {
    std::iter::repeat('#').take(width).collect()
}

const MAX_COL_WIDTH: usize = 70;

fn main() {
    let args = init::handle_opts();
    let all_items = get_all_items(&args);
    let hist = stats::get_histogram(all_items.iter());

    for tag_stats in generate_tag_stats(&all_items) {
        println!("{:.2}±{:.2} {}: {}/{}", tag_stats.avg, tag_stats.stdev,
                 tag_stats.tag, tag_stats.rated, tag_stats.total);
    }
    println!("");

    let (_, max_rated) = hist.get_max_rated();
    for rating in 1..(MAX_RATING + 1) {
        let rated = hist[Some(rating)];
        let num = (rated as f32 / max_rated as f32 *
                   MAX_COL_WIDTH as f32).round() as usize;
        let bar = generate_bar(num) + if num > 0 { " " } else { "" };
        println!("{:>2}: {}{}", rating, bar, rated);
    }
    println!("rated: {}/{}", hist.get_all_rated(), all_items.len());
    let (avg, stdev) = hist.get_avg_and_stdev();
    println!("rating: {:.2}±{:.2}", avg, stdev);
}
