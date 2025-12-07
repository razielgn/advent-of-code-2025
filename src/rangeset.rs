// Adapted from https://github.com/dlight/todomd/blob/d9ef5920097a089bba553130711b29d50cc7d6b6/common/todomd/src/rangeset.rs

use itertools::{Itertools, Position};
use std::ops::{Bound, Range};

trait RangeExt {
    /// Merge ranges. If ranges are adjacent or overlapping, returns a single range that covers them. If they are not, returns None.
    fn merge_range(&self, range: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl<T: PartialOrd + Ord + Copy> RangeExt for Range<T> {
    fn merge_range(&self, range: &Self) -> Option<Self> {
        assert!(!self.is_empty());
        assert!(!range.is_empty());

        let start = self.start.min(range.start);
        let end = self.end.max(range.end);

        if self.end < range.start || range.end < self.start {
            None
        } else {
            Some(start..end)
        }
    }
}

/// A set of non-overlapping, non-adjacent ranges. Inserting new ranges will merge them if
/// possible. Ranges are kept sorted.
#[derive(Debug, Default)]
pub struct RangeSet<T> {
    contents: Vec<Range<T>>,
}

impl<T: PartialOrd + Copy + Ord + Clone> RangeSet<T> {
    pub fn insert_range(&mut self, range: &Range<T>) {
        if self.contents.is_empty() {
            self.contents.push(range.clone());
            return;
        }

        // searching for start:
        //  Err(0)       Ok(0)         Err(1)        Ok(1)         Err(2)       Ok(2)
        //  Included(0)  Included(0)   Included(1)   Included(1)   Included(2)  Included(2)
        // [             0,                          1,                         2]

        let start =
            match self.contents.binary_search_by(|x| x.end.cmp(&range.start)) {
                Ok(idx) | Err(idx) => Bound::Included(idx),
            };

        // searching for end:
        //  Err(0)       Ok(0)         Err(1)        Ok(1)         Err(2)      Ok(2)
        //  Excluded(0)  Included(0)   Excluded(1)   Included(1)   Excluded(2) Included(2)
        // [             0,                          1,                        2]

        let end =
            match self.contents.binary_search_by(|x| x.start.cmp(&range.end)) {
                Ok(idx) => Bound::Included(idx),
                Err(idx) => Bound::Excluded(idx),
            };

        let mut new_range = range.clone();

        // Merging with just the endpoints is sufficient, because the middle values are absorbed by
        // the inserted range
        let endpoints = self.contents[(start, end)]
            .iter()
            .with_position()
            .filter_map(|(p, r)| (!matches!(p, Position::Middle)).then_some(r));

        for inner_range in endpoints {
            new_range = new_range.merge_range(inner_range).unwrap();
        }

        self.contents.splice((start, end), [new_range]);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Range<T>> {
        self.contents.iter()
    }
}

#[cfg(test)]
impl<T: PartialOrd + Copy + Ord + Default> From<Vec<Range<T>>> for RangeSet<T> {
    fn from(vec: Vec<Range<T>>) -> Self {
        vec.into_iter().collect()
    }
}

#[cfg(test)]
impl<T: PartialOrd + Copy + Ord + Clone + Default> FromIterator<Range<T>>
    for RangeSet<T>
{
    fn from_iter<I: IntoIterator<Item = Range<T>>>(iter: I) -> Self {
        let mut range = Self::default();

        for v in iter {
            range.insert_range(&v);
        }

        range
    }
}

#[cfg(test)]
#[allow(clippy::single_range_in_vec_init)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_merge_ranges() {
        assert_eq!((1..2).merge_range(&(0..1)), Some(0..2));
        assert_eq!((0..5).merge_range(&(5..10)), Some(0..10));
        assert_eq!((0..5).merge_range(&(6..10)), None);
        assert_eq!((0..5).merge_range(&(5..10)), Some(0..10));
        assert_eq!((0..5).merge_range(&(5..10)), Some(0..10));
        assert_eq!((430..888).merge_range(&(602..835)), Some(430..888));
        assert_eq!(
            (usize::MAX - 1..usize::MAX)
                .merge_range(&(usize::MAX - 2..usize::MAX - 1)),
            Some(usize::MAX - 2..usize::MAX)
        );
    }

    #[test]
    fn random_merge_ranges() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
        for _ in 0..1000 {
            let start1 = rng.random_range(0..1000);
            let start2 = rng.random_range(0..1000);
            let end1 = rng.random_range(0..1000);
            let end2 = rng.random_range(0..1000);
            let range1 = start1..end1;
            let range2 = start2..end2;
            if range1.is_empty() || range2.is_empty() {
                continue;
            }
            let merged = range1.merge_range(&range2);
            if start1 < start2 {
                if end1 >= start2 {
                    if end2 >= end1 {
                        assert_eq!(merged, Some(start1..end2));
                    } else {
                        assert_eq!(merged, Some(start1..end1));
                    }
                } else {
                    assert_eq!(merged, None);
                }
            } else if end2 >= start1 {
                if end1 >= end2 {
                    assert_eq!(merged, Some(start2..end1));
                } else {
                    assert_eq!(merged, Some(start2..end2));
                }
            } else {
                assert_eq!(merged, None);
            }
        }
    }

    #[test]
    fn test_range_set() {
        assert_eq!(RangeSet::from(vec![0..1, 1..2]).contents, vec![0..2]);
        assert_eq!(RangeSet::from(vec![1..2, 0..1]).contents, vec![0..2]);
        assert_eq!(RangeSet::from(vec![0..1, 2..3]).contents, vec![0..1, 2..3]);
        assert_eq!(RangeSet::<u8>::from(vec![]).contents, vec![]);
        assert_eq!(
            RangeSet::from(vec![5..10, 0..3, 8..15]).contents,
            vec![0..3, 5..15]
        );

        assert_eq!(
            RangeSet::from(vec![
                usize::MAX - 1..usize::MAX,
                usize::MAX - 2..usize::MAX - 1
            ])
            .contents,
            vec![usize::MAX - 2..usize::MAX]
        );

        assert_eq!(
            RangeSet::from(vec![1..3, 5..7, 9..11, 13..15, 6..10]).contents,
            vec![1..3, 5..11, 13..15]
        );

        assert_eq!(
            RangeSet::from(vec![14..21, 0..5, 10..13, 20..25, 4..11]).contents,
            vec![0..13, 14..25]
        );

        assert_eq!(
            RangeSet::from(vec![1..4, 3..6, 5..8, 7..10, 9..12]).contents,
            vec![1..12]
        );

        assert_eq!(
            RangeSet::from(vec![
                100..200,
                300..400,
                500..600,
                150..350,
                450..550
            ])
            .contents,
            vec![100..400, 450..600]
        );

        assert_eq!(
            RangeSet::from(vec![10..20, 1..2, 2..3, 3..4, 1..6, 4..5, 5..6])
                .contents,
            vec![1..6, 10..20]
        );
        assert_eq!(
            RangeSet::from(vec![1000..2000, 3000..4000, 0..5000]).contents,
            vec![0..5000]
        );

        assert_eq!(
            RangeSet::from(vec![1..5, 10..15, 20..25, 2..3, 12..13, 22..23])
                .contents,
            vec![1..5, 10..15, 20..25]
        );

        assert_eq!(
            RangeSet::from(vec![1..10, 20..30, 40..50, 5..45]).contents,
            vec![1..50]
        );
    }
}
