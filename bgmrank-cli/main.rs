extern crate enum_set;
extern crate getopts;
extern crate libbgmrank;

mod init;

use libbgmrank::{Histogram, Item, MAX_RATING};

fn get_all_items(args: &init::Args) -> Vec<Item> {
    let mut result = vec![];
    for category in args.categories.iter() {
        for state in args.states.iter() {
            println!("fetching {}: {}/{}", args.username, category, state);
            result.extend(libbgmrank::get_items(
                &args.username,
                category,
                state,
                |page| {
                    println!("  fetching page {}...", page);
                },
            ));
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
    let hist: Histogram = all_items.iter().collect();

    for tag_stats in libbgmrank::generate_tag_stats(&all_items) {
        println!(
            "{} {}: {}/{}",
            tag_stats.stats.rating, tag_stats.tag, tag_stats.stats.rated, tag_stats.stats.total
        );
    }
    println!("");

    let (_, max_rated) = hist.get_max_rated();
    let stats = hist.get_stats();
    for rating in 1..(MAX_RATING + 1) {
        let rated = hist[Some(rating)];
        let num = (rated as f32 / max_rated as f32 * MAX_COL_WIDTH as f32).round() as usize;
        let bar = generate_bar(num) + if num > 0 { " " } else { "" };
        println!("{:>2}: {}{}", rating, bar, rated);
    }
    println!("rated: {}/{}", stats.rated, stats.total);
    println!("rating: {}", stats.rating);
}
