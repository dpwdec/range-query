use num::Bounded;

fn main() {
    let x = vec!(4, 2, 3, 9, 10);
    let y = x.x_min();
    println!("{}", y);
}

trait Min<T> where T: PartialOrd + Bounded + Clone {
    fn x_min(&self) -> T;
}

impl<T> Min<T> for [T] where T: PartialOrd + Bounded + Clone {
    fn x_min(&self) -> T {
        self.iter().fold(T::max_value(), |acc, x| if x < &acc { x.clone() } else { acc })
    }
}

fn pre_process_collection<T>(x: Vec<T>) -> Vec<T> where T: PartialOrd + Bounded + Clone {
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
        assert_eq!(vec!(2, 0, 21), pre_process_collection(x));
    }
}