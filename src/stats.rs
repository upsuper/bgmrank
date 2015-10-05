use std::cmp::{PartialOrd, Ordering};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use data::{Rating, Item, MAX_RATING};

#[derive(PartialEq)]
pub struct Interval {
    pub avg: f32,
    pub stdev: f32
}

impl Interval {
    pub fn is_nan(&self) -> bool {
        assert!(self.avg.is_nan() == self.stdev.is_nan());
        self.avg.is_nan()
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.avg, -self.stdev).partial_cmp(&(other.avg, -other.stdev))
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.2}±{:.2}", self.avg, self.stdev)
    }
}

pub struct Stats {
    pub total: usize,
    pub rated: usize,
    pub rating: Interval
}

pub struct Histogram {
    ratings: [usize; MAX_RATING as usize + 1]
}

impl Histogram {
    pub fn get_max_rated(&self) -> (Rating, usize) {
        self.ratings.iter().enumerate().skip(1)
            .fold((0, 0), |(cur_rating, cur_max), (rating, &num)| {
                if num > cur_max {
                    (rating as Rating, num)
                } else {
                    (cur_rating, cur_max)
                }
            })
    }
    pub fn get_stats(&self) -> Stats {
        let (rated, sum) = self.ratings.iter().enumerate().skip(1)
            .fold((0, 0), |(count, sum), (rating, &num)| {
                (count + num, sum + (rating as usize * num))
            });
        let avg = sum as f32 / rated as f32;
        let var = self.ratings.iter().enumerate().skip(1)
            .fold(0f32, |sum, (rating, &num)| {
                sum + num as f32 * (rating as f32 - avg).powf(2.0)
            }) / rated as f32;
        Stats {
            total: rated + self.ratings[0],
            rated: rated,
            rating: Interval { avg: avg, stdev: var.sqrt() }
        }
    }
}

impl<'a> FromIterator<&'a Item> for Histogram {
    fn from_iter<Iter>(iter: Iter) -> Self
            where Iter: IntoIterator<Item=&'a Item> {
        let mut result = Histogram { ratings: [0; MAX_RATING as usize + 1] };
        for item in iter {
            result[item.rating] += 1;
        }
        result
    }
}

impl Index<Option<Rating>> for Histogram {
    type Output = usize;
    fn index<'a>(&'a self, rating: Option<Rating>) -> &'a Self::Output {
        match rating {
            Some(rating) => &self.ratings[rating as usize],
            None => &self.ratings[0]
        }
    }
}

impl IndexMut<Option<Rating>> for Histogram {
    fn index_mut<'a>(&'a mut self, rating: Option<Rating>)
            -> &'a mut Self::Output {
        match rating {
            Some(val) => &mut self.ratings[val as usize],
            None => &mut self.ratings[0]
        }
    }
}
