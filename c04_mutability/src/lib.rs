#[cfg(test)]
mod c_mutability {
    #[test]
    fn simple() {
        let mut x = 0;

        assert_eq!(x, 0);

        x += 1;

        assert_eq!(x, 1);
    }
}
