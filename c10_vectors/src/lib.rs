#[cfg(test)]
mod vector {
    #[test]
    fn create_basic() {
        let v: Vec<u8> = Vec::new();

        assert!(v.is_empty());
    }

    #[test]
    fn create_macro() {
        let v: Vec<u8> = vec![];

        assert!(v.is_empty());
    }

    #[test]
    fn create_macro_with_elements() {
        let v: Vec<u8> = vec![1, 2, 3];

        assert!(v.len() == 3);
    }

    #[test]
    fn create_macro_with_repeating_elements() {
        let v: Vec<u8> = vec![4; 10]; // [4, 4, ..., 4]

        assert!(v.len() == 10);
        assert!(v[9] == 4);
    }

    #[test]
    fn create_with_capacity() {
        let v: Vec<u8> = Vec::with_capacity(5); // no reallocation until 5 elements

        assert!(v.capacity() == 5);
        assert!(v.is_empty());
    }

    #[test]
    fn add_element_basic() {
        let mut v: Vec<u8> = Vec::new();

        v.push(1);

        assert!(v.len() == 1);
        assert!(v[0] == 1);
    }

    #[test]
    fn convert_array_to_vector() {
        let arr = [1, 2, 3];
        let v: Vec<i32> = arr.to_vec(); // `v` does not own `arr`

        assert!(v == vec![1, 2, 3]);
    }

    #[test]
    fn collect_range_into_vector() {
        let v: Vec<i32> = (1..4).collect();

        assert!(v == vec![1, 2, 3]);
    }
}
