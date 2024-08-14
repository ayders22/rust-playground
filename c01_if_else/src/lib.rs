#[cfg(test)]
mod if_else {
    #[test]
    fn if_basic() {
        let x = 5;
        let pass1;
        let mut pass2 = false;

        if x > 5 {
            pass1 = false;
        } else if x < 5 {
            pass1 = false;
        } else {
            pass1 = true;
        }

        if x >= 5 {
            pass2 = true;
        }

        assert!(pass1 == true && pass2 == true);
    }

    #[test]
    fn if_as_expression() {
        // if can be an expression meaning it can return a value
        let x = 5;
        let y = if x == 5 { 0 } else { 100 };

        assert!(y == 0 && x == 5);
    }

    #[test]
    fn if_to_unpack_option() {
        let x = Some(5);
        let pass1;
        let pass2;

        if let Some(y) = x {
            pass1 = true;
            assert!(y == 5);
        } else {
            // x is None
            pass1 = false;
        }

        if let Some(5) = x {
            pass2 = true;
        } else if let Some(4) = x {
            pass2 = false;
        } else {
            pass2 = false;
        }

        assert!(pass1 == true && pass2 == true);
    }

    #[test]
    fn if_to_unpack_result() {
        let x: Result<i32, i32> = Ok(5);
        let pass;

        if let Ok(y) = x {
            pass = true;
            assert!(y == 5);
        } else {
            pass = false;
        }

        assert!(pass == true);
    }
}
