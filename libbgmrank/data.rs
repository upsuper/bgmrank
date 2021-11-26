use enumset::EnumSetType;
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Display, EnumIter, EnumSetType, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Category {
    Anime,
    Book,
    Music,
    Game,
    Real,
}

#[derive(Display, EnumIter, EnumSetType, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Wish,
    Collect,
    Do,
    OnHold,
    Dropped,
}

pub type Id = u32;
pub type Rating = u8;
pub const MAX_RATING: Rating = 10;

#[derive(Default)]
pub struct Item {
    pub id: Id,
    pub title: String,
    pub rating: Option<Rating>,
    pub tags: Vec<String>,
}
