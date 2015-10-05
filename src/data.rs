use std;
use std::str::FromStr;

use enum_set::CLike;

pub trait ToStaticStr {
    fn to_static_str(&self) -> &'static str;
}

pub trait ListAll {
    fn list_all() -> &'static [Self];
}

macro_rules! static_str_enum {
    (
        $name:ident {
            $($item:ident => $str:ident),*
        }
    ) => {
        #[derive(Clone, Copy)]
        #[repr(u32)]
        pub enum $name {
            $($item),*
        }
        impl ToStaticStr for $name {
            fn to_static_str(&self) -> &'static str {
                match *self {
                    $($name::$item => stringify!($str)),*
                }
            }
        }
        impl ListAll for $name {
            fn list_all() -> &'static [Self] {
                static LIST: &'static [$name] = &[$($name::$item),+];
                &LIST
            }
        }
        impl FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($str) => Ok($name::$item)),*,
                    _ => Err(())
                }
            }
        }
        impl CLike for $name {
            fn to_u32(&self) -> u32 {
                *self as u32
            }
            unsafe fn from_u32(v: u32) -> $name {
                std::mem::transmute(v)
            }
        }
    }
}

static_str_enum! {
    Category {
        Anime => anime,
        Book => book,
        Music => music,
        Game => game,
        Real => real
    }
}
static_str_enum! {
    State {
        Wish => wish,
        Collect => collect,
        Do => do,
        OnHold => on_hold,
        Dropped => dropped
    }
}

pub type Id = u32;
pub type Rating = u8;
pub const MAX_RATING: Rating = 10;

pub struct Item {
    pub id: Id,
    pub title: String,
    pub rating: Option<Rating>,
    pub tags: Vec<String>
}
