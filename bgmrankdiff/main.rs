use colored::Colorize;
use either::Either;
use libbgmrank::{Category, Rating, State};
use std::collections::HashMap;
use std::fmt;
use structopt::StructOpt;
use unicode_width::UnicodeWidthStr;

#[derive(StructOpt)]
struct Opts {
    #[structopt(name = "USER1")]
    user1: String,
    #[structopt(name = "USER2")]
    user2: String,
}

fn main() {
    let opts = Opts::from_args();
    println!("fetching {}:", &opts.user1);
    let items1 = libbgmrank::get_items(&opts.user1, Category::Anime, State::Collect, |page| {
        println!("  fetching page {}...", page);
    });
    println!("fetching {}:", &opts.user2);
    let items2 = libbgmrank::get_items(&opts.user2, Category::Anime, State::Collect, |page| {
        println!("  fetching page {}...", page);
    });
    println!();

    let map1 = items1
        .iter()
        .map(|item| (item.id, item))
        .collect::<HashMap<_, _>>();
    let map2 = items2
        .iter()
        .map(|item| (item.id, item))
        .collect::<HashMap<_, _>>();
    let mut list = Iterator::chain(
        items1.iter().map(Either::Left),
        items2.iter().map(Either::Right),
    )
    .collect::<Vec<_>>();
    list.sort_by(|&a, &b| Ord::cmp(&a.rating, &b.rating).reverse());

    for item in list {
        let line = match item {
            Either::Left(item) => match map2.get(&item.id) {
                Some(&item2) => format!(
                    "~ {} {}..{}",
                    PadTitle(&item.title),
                    FormatRating(item.rating),
                    FormatRating(item2.rating),
                )
                .yellow(),
                None => format!(
                    "- {} {}..",
                    PadTitle(&item.title),
                    FormatRating(item.rating)
                )
                .red(),
            },
            Either::Right(item) => {
                if map1.contains_key(&item.id) {
                    continue;
                }
                format!(
                    "+ {}   ..{}",
                    PadTitle(&item.title),
                    FormatRating(item.rating),
                )
                .green()
            }
        };
        println!("{}", line);
    }
}

const TITLE_WIDTH: usize = 20;

struct PadTitle<'a>(&'a str);

impl<'a> fmt::Display for PadTitle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.0.width();
        f.write_str(self.0)?;
        if width <= TITLE_WIDTH {
            for _ in 0..TITLE_WIDTH - width {
                f.write_str(" ")?;
            }
        } else {
            f.write_str("\n")?;
            // Extra 2 characters for the leading symbol
            for _ in 0..TITLE_WIDTH + 2 {
                f.write_str(" ")?;
            }
        }
        Ok(())
    }
}

struct FormatRating(Option<Rating>);

impl fmt::Display for FormatRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            None => f.write_str(" ?"),
            Some(r) => write!(f, "{:2}", r),
        }
    }
}
