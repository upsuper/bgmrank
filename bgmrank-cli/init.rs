use std;
use std::str::FromStr;

use enum_set::{CLike, EnumSet};
use getopts::{Matches, Options};

use libbgmrank::{Category, ListAll, State, ToStaticStr};

fn get_args() -> (String, Vec<String>) {
    let mut args = std::env::args();
    (args.next().unwrap(), args.collect())
}

fn list_enum_str<E: 'static + ListAll + ToStaticStr>() -> Vec<&'static str> {
    E::list_all().iter().map(|e| e.to_static_str()).collect()
}

fn get_opts() -> Options {
    let mut opts = Options::new();
    opts.optmulti(
        "c",
        "category",
        &list_enum_str::<Category>().join(", "),
        "CAT",
    );
    opts.optmulti("s", "state", &list_enum_str::<State>().join(", "), "STATE");
    opts.optflag("h", "help", "print this help menu");
    opts
}

fn show_usage_and_exit(program: String, opts: Options, code: i32) -> ! {
    let brief = format!("Usage: {} [options] username", program);
    print!("{}", opts.usage(&brief));
    std::process::exit(code)
}

pub struct Args {
    pub username: String,
    pub categories: EnumSet<Category>,
    pub states: EnumSet<State>,
}

fn process_opt_list<E: ToStaticStr + FromStr + CLike>(
    name: &'static str,
    item_list: Vec<String>,
    default_value: E,
) -> Result<EnumSet<E>, String> {
    let mut result = EnumSet::new();
    for item in item_list {
        match item.parse::<E>() {
            Ok(value) => {
                result.insert(value);
            }
            Err(_) => {
                return Err(format!("unknown {} '{}'", name, item));
            }
        }
    }
    if result.is_empty() {
        result.insert(default_value);
    }
    Ok(result)
}

fn parse_opts(mut matches: Matches) -> Result<Args, String> {
    if matches.free.len() != 1 {
        return Err(String::from("username not specified"));
    }
    Ok(Args {
        username: matches.free.remove(0),
        categories: process_opt_list::<Category>(
            "category",
            matches.opt_strs("c"),
            Category::Anime
        )?,
        states: process_opt_list::<State>(
            "state",
            matches.opt_strs("s"),
            State::Collect
        )?,
    })
}

pub fn handle_opts() -> Args {
    let (program, args) = get_args();
    let opts = get_opts();
    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            show_usage_and_exit(program, opts, 2);
        }
    };
    if matches.opt_present("h") {
        show_usage_and_exit(program, opts, 0);
    }
    match parse_opts(matches) {
        Ok(result) => result,
        Err(msg) => {
            println!("Error: {}", msg);
            show_usage_and_exit(program, opts, 2);
        }
    }
}
