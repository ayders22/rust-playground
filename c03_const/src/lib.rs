#[cfg(test)]
mod c_const {
    mod const_global {
        const PI: f64 = 3.14;
        const UNIT: &str = "cm";

        #[test]
        fn const_global() {
            let diameter = 2;
            let perimeter = f64::from(2) * PI * f64::from(diameter); // 2*pi*r

            assert_eq!(perimeter, 12.56);
            assert_eq!(UNIT, "cm");
        }
    }

    #[test]
    fn const_local() {
        const PI: f64 = 3.14;
        const UNIT: &str = "mm";

        let diameter = 3;
        let perimeter = f64::from(2) * PI * f64::from(diameter); // 2*pi*r

        assert_eq!(perimeter, 18.84);
        assert_eq!(UNIT, "mm");
    }
}
