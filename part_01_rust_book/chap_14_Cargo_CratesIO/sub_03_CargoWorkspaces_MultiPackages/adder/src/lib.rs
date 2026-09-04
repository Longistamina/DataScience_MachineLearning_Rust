pub fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
