fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_trivial() {
        let sum = 3 + 3;
        assert!(sum == 6);
    }

    #[test]
    #[should_panic]
    fn test_trivial_failure() {
        let sum = 3 + 3;
        assert!(sum == 7);
    }
}
