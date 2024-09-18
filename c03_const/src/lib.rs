#[cfg(test)]
mod c_const {
    mod m1 {
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

    mod m2 {
        struct Circle {
            diameter: f64,
        }

        impl Circle {
            const PI: f64 = 3.14;

            fn perimeter(&self) -> f64 {
                f64::from(2) * Circle::PI * self.diameter
            }
        }

        #[test]
        fn const_with_struct() {
            let circle = Circle { diameter: 2.0 };

            assert_eq!(circle.perimeter(), 12.56);
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

    #[test]
    fn const_as_array_size() {
        const SIZE: usize = 5;
        let arr: [i32; SIZE] = [1, 2, 3, 4, 5];

        assert_eq!(arr.len(), SIZE);
    }
}
