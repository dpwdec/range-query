use num::Bounded;

fn main() {
    let x = vec!(4, 2, 3, 9, 10);
    let y = x.x_min();
    println!("{}", y);
}

trait Min<T> where T: PartialOrd + Bounded + Clone {
    fn x_min(&self) -> T;
}

impl<T> Min<T> for Vec<T> where T: PartialOrd + Bounded + Clone {
    fn x_min(&self) -> T {
        self.iter().fold(T::max_value(), |acc, x| if x < &acc { x.clone() } else { acc })
    }
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
}