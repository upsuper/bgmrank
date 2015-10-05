extern crate enum_set;
extern crate getopts;
extern crate hyper;
extern crate kuchiki;
extern crate selectors;
#[macro_use] extern crate string_cache;

mod data;
mod init;
mod helpers;
mod parser;
mod stats;

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

fn generate_bar(width: usize) -> String {
    std::iter::repeat('#').take(width).collect()
}

const MAX_COL_WIDTH: usize = 70;

fn main() {
    let args = init::handle_opts();
    let all_items = get_all_items(&args);
    let hist = stats::get_histogram(all_items.iter());

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
