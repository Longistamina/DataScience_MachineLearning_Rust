#![allow(unused_imports)]
use rand;

pub fn subtract<T: std::ops::Sub<Output = T>>(x: T, y: T) -> T {
    x - y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = subtract(2, 2);
        assert_eq!(result, 0);
    }
}
