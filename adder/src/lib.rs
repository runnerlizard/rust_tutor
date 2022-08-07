#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_doesnt_works() {
        let result = 1 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn panic_test() {
        panic!("Failed test")
    }
}
