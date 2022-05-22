use num::{Bounded, iter::RangeFrom};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

fn main() {
}

struct RangeQuery<T> where T: PartialOrd + Bounded + Clone + Hash {
    x_min_map: HashMap<u64, Vec<T>>
}

impl<T> RangeQuery<T> where T: PartialOrd + Bounded + Clone + Hash {
    fn new() -> RangeQuery<T> {
        RangeQuery { x_min_map: HashMap::<u64, Vec<T>>::new() }
    }

    fn x_minimum_range(&mut self, arr: &Vec<T>, start: usize, end: usize) -> T {
        let base = (arr.len() as f64).sqrt().ceil() as usize;

        // check values are in the same sub range
        if  (start as f64 / base as f64).floor() == (end as f64 / base as f64).floor() {
            return arr[start..=end].x_min()
        }

        let mut hash = DefaultHasher::new();
        arr.hash(&mut hash);
        let decomp_arr  = match self.x_min_map.get(&hash.finish()) {
            Some(decomp) => decomp,
            None => {
                let new_decomp = pre_process_collection(arr);
                let mut new_hash = DefaultHasher::new();
                new_decomp.hash(&mut new_hash);
                let key = new_hash.finish();
                self.x_min_map.insert(key, pre_process_collection(arr));
                self.x_min_map.get(&key).unwrap()
            }
        };

        let mut cmp_values: Vec<T> = Vec::new();

        let decomp_start_end_proximity = (end as f64 / base as f64).floor() - (start as f64 / base as f64).floor();

        let adjusted_start = if start % base != 0 {
            cmp_values.append(&mut arr[start..(start + (base - start % base))].to_vec());
            start + (base - start % base)
        } else {
            start
        };
        
        let adjusted_end = if end % base != base - 1 {
            cmp_values.append(&mut arr[(end - (end % base))..=end].to_vec());
            end - ((end % base) + 1)
        } else {
            end
        };

        // check that the start and end ranges are not neighbors within a base
        if decomp_start_end_proximity == 1.0 { return cmp_values.x_min() }

        let in_range_start = adjusted_start / base;
        let in_range_end = (adjusted_end as f64 / base as f64).floor() as usize;

        cmp_values.append(&mut decomp_arr[in_range_start..=in_range_end].to_vec());

        cmp_values.x_min()
    }
}

trait Min<T> where T: PartialOrd + Bounded + Clone {
    fn x_min(&self) -> T;
}

impl<T> Min<T> for [T] where T: PartialOrd + Bounded + Clone {
    fn x_min(&self) -> T {
        self.iter().fold(T::max_value(), |acc, x| if x < &acc { x.clone() } else { acc })
    }
}

fn pre_process_collection<T>(x: &Vec<T>) -> Vec<T> where T: PartialOrd + Bounded + Clone {
    let chunk_size = (x.len() as f64).sqrt().ceil() as usize;
    x
        .chunks(chunk_size)
        .fold(Vec::new(), |mut acc, chunk| {
            acc.push(chunk.x_min());
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_int_collection() {
        let x = vec!(4, 2, 3, 9, 10, 14, 12, 1);
        assert_eq!(1, x.x_min());
    }

    #[test]
    fn test_signed_int_collection() {
        let x = vec!(4, -2, 3, 9, -10, 14, 12, -1);
        assert_eq!(-10, x.x_min());
    }

    #[test]
    fn test_float_collection() {
        let x = vec!(4.3, -2.9, 3.0, 9.0, -10.5, -10.6, 12.4, -1.0);
        assert_eq!(-10.6, x.x_min());
    }

    #[test]
    fn test_pre_process_collection() {
        let x = vec!(2, 3, 5, 7, 100, 0, 30, 21);
        assert_eq!(vec!(2, 0, 21), pre_process_collection(&x));
    }

    #[test]
    fn test_find_neighboring_min() {
        let mut range_query = RangeQuery::<usize>::new();
        let arr = vec![2, 3, 5, 7, 100, 0, 30, 21, 1];
        assert_eq!(3, range_query.x_minimum_range(&arr, 1, 4));
    }

    #[test]
    fn test_find_range_min() {
        let mut range_query = RangeQuery::<usize>::new();
        let arr = vec![
                                2,   3,  5,  7, 
                                100, 0,  30, 21, 
                                91,  70, 33, 400, 
                                50,  31, 16
        ];
        assert_eq!(16, range_query.x_minimum_range(&arr, 6, 14));
    }

    #[test]
    fn test_find_single_range() {
        let mut range_query = RangeQuery::<usize>::new();
        let arr = vec![
                                2,   3,  5,  7, 
                                100, 55, 30, 21, 
                                91,  70, 33, 400, 
                                50,  31, 16
        ];
        assert_eq!(21, range_query.x_minimum_range(&arr, 4, 7));
    }

    #[test]
    fn test_find_range_at_end() {
        let mut range_query = RangeQuery::<usize>::new();
        let arr = vec![
                                2,   3,  5,  7, 
                                100, 55, 30, 21, 
                                91,  70, 33, 400, 
                                50,  31, 1,  22
        ];
        assert_eq!(1, range_query.x_minimum_range(&arr, 0, 15));
    }
}