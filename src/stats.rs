use std::ops::{Index, IndexMut};
use data::{Rating, Item, MAX_RATING};

pub struct Histogram {
    ratings: [usize; MAX_RATING as usize + 1]
}

impl Histogram {
    pub fn get_all_rated(&self) -> usize {
        self.ratings.iter().skip(1).fold(0, |acc, &item| acc + item)
    }
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
    fn get_avg_and_count(&self) -> (f32, usize) {
        let (count, sum) = self.ratings.iter().enumerate().skip(1)
            .fold((0, 0), |(count, sum), (rating, &num)| {
                (count + num, sum + (rating as usize * num))
            });
        (sum as f32 / count as f32, count)
    }
    pub fn get_avg_and_stdev(&self) -> (f32, f32) {
        let (avg, count) = self.get_avg_and_count();
        let var = self.ratings.iter().enumerate().skip(1)
            .fold(0f32, |sum, (rating, &num)| {
                sum + num as f32 * (rating as f32 - avg).powf(2.0)
            }) / count as f32;
        (avg, var.sqrt())
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

pub fn get_histogram<'a, Iter>(iter: Iter) -> Histogram 
        where Iter: Iterator<Item=&'a Item> {
    let mut result = Histogram { ratings: [0; MAX_RATING as usize + 1] };
    for item in iter {
        result[item.rating] += 1;
    }
    result
}
