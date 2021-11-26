use crate::classifier;
use crate::data::{Item, Rating, MAX_RATING};
use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

#[derive(PartialEq)]
pub struct Interval {
    pub avg: f32,
    pub stdev: f32,
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
    pub rating: Interval,
}

pub struct Histogram {
    ratings: [usize; MAX_RATING as usize + 1],
}

impl Histogram {
    pub fn get_max_rated(&self) -> (Rating, usize) {
        self.ratings.iter().enumerate().skip(1).fold(
            (0, 0),
            |(cur_rating, cur_max), (rating, &num)| {
                if num > cur_max {
                    (rating as Rating, num)
                } else {
                    (cur_rating, cur_max)
                }
            },
        )
    }
    pub fn get_stats(&self) -> Stats {
        let (rated, sum) = self
            .ratings
            .iter()
            .enumerate()
            .skip(1)
            .fold((0, 0), |(count, sum), (rating, &num)| {
                (count + num, sum + (rating as usize * num))
            });
        let avg = sum as f32 / rated as f32;
        let var = self
            .ratings
            .iter()
            .enumerate()
            .skip(1)
            .fold(0f32, |sum, (rating, &num)| {
                sum + num as f32 * (rating as f32 - avg).powf(2.0)
            })
            / rated as f32;
        Stats {
            total: rated + self.ratings[0],
            rated,
            rating: Interval {
                avg,
                stdev: var.sqrt(),
            },
        }
    }
}

impl<'a> FromIterator<&'a Item> for Histogram {
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: IntoIterator<Item = &'a Item>,
    {
        let mut result = Histogram {
            ratings: [0; MAX_RATING as usize + 1],
        };
        for item in iter {
            result[item.rating] += 1;
        }
        result
    }
}

impl Index<Option<Rating>> for Histogram {
    type Output = usize;
    fn index(&self, rating: Option<Rating>) -> &Self::Output {
        match rating {
            Some(rating) => &self.ratings[rating as usize],
            None => &self.ratings[0],
        }
    }
}

impl IndexMut<Option<Rating>> for Histogram {
    fn index_mut(&mut self, rating: Option<Rating>) -> &mut Self::Output {
        match rating {
            Some(val) => &mut self.ratings[val as usize],
            None => &mut self.ratings[0],
        }
    }
}

pub struct TagStats {
    pub tag: String,
    pub stats: Stats,
}

pub fn generate_tag_stats(all_items: &[Item]) -> Vec<TagStats> {
    let mut result: Vec<TagStats> = classifier::classify_by_tags(all_items)
        .into_iter()
        .filter_map(|(tag, items)| {
            let hist: Histogram = items.into_iter().collect();
            let stats = hist.get_stats();
            if stats.rating.is_nan() {
                return None;
            }
            Some(TagStats { tag, stats })
        })
        .collect();
    result.sort_by(|l, r| {
        // It should be safe to unwrap here because we should have
        // filtered out all NaNs in the loop above.
        l.stats
            .rating
            .partial_cmp(&r.stats.rating)
            .unwrap()
            .reverse()
    });
    result
}

#[cfg(test)]
mod test {
    use super::Histogram;
    use crate::data::Item;
    use float_cmp::ApproxEqUlps;

    macro_rules! item_with_rating {
        ($rating:expr) => {
            Item {
                rating: $rating,
                ..Default::default()
            }
        };
    }

    #[test]
    fn test_generate_histogram() {
        let items = vec![
            item_with_rating!(Some(4)),
            item_with_rating!(None),
            item_with_rating!(Some(2)),
            item_with_rating!(Some(5)),
            item_with_rating!(Some(1)),
            item_with_rating!(Some(10)),
            item_with_rating!(Some(4)),
            item_with_rating!(None),
            item_with_rating!(Some(2)),
            item_with_rating!(Some(2)),
            item_with_rating!(Some(9)),
            item_with_rating!(Some(2)),
            item_with_rating!(Some(3)),
            item_with_rating!(Some(6)),
            item_with_rating!(None),
            item_with_rating!(Some(9)),
            item_with_rating!(Some(1)),
            item_with_rating!(Some(5)),
            item_with_rating!(Some(6)),
            item_with_rating!(None),
        ];
        let hist = items.iter().collect::<Histogram>();
        assert_eq!(hist.ratings, [4, 2, 4, 1, 2, 2, 2, 0, 0, 2, 1]);
    }

    fn get_test_histogram() -> Histogram {
        Histogram {
            ratings: [200, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110],
        }
    }

    #[test]
    fn test_histogram_indexing() {
        let hist = get_test_histogram();
        assert_eq!(hist[None], 200);
        for i in 1..10 {
            assert_eq!(hist[Some(i)], 100 + i as usize);
        }
        let mut hist = get_test_histogram();
        hist[None] *= 2;
        assert_eq!(hist[None], 400);
        for i in 1..10 {
            let m = i as usize + 1;
            hist[Some(i)] *= m;
            assert_eq!(hist[Some(i)], (100 + i as usize) * m);
        }
    }

    #[test]
    fn test_histogram_max_rated() {
        let hist = Histogram {
            ratings: [10, 1, 2, 5, 5, 4, 8, 8, 7, 0, 5],
        };
        assert_eq!(hist.get_max_rated(), (6, 8));
    }

    #[test]
    fn test_histogram_stats() {
        let hist = Histogram {
            ratings: [60, 82, 60, 76, 26, 4, 69, 40, 85, 67, 96],
        };
        let stats = hist.get_stats();
        assert_eq!(stats.total, 665);
        assert_eq!(stats.rated, 605);
        assert!(stats.rating.avg.approx_eq_ulps(&5.77024793388, 1));
        assert!(stats.rating.stdev.approx_eq_ulps(&3.22415072111, 1));

        let hist = Histogram {
            ratings: [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let stats = hist.get_stats();
        assert_eq!(stats.rated, 0);
        assert!(stats.rating.is_nan());
    }
}
