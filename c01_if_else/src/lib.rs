#[cfg(test)]
mod if_else {
    #[test]
    fn if_without_else() {
        let x = 5;
        let mut pass = false;

        if x >= 5 {
            pass = true;
        }

        assert!(pass == true && x == 5);
    }

    #[test]
    fn if_else() {
        let x = 5;
        let pass;

        if x != 5 {
            pass = false;
        } else {
            pass = true;
        }

        assert!(pass == true && x == 5);
    }

    #[test]
    fn if_else_if_else() {
        let x = 5;
        let pass;

        if x > 5 {
            pass = false;
        } else if x < 5 {
            pass = false;
        } else {
            pass = true;
        }

        assert!(pass == true && x == 5);
    }

    #[test]
    fn if_as_expression() {
        // if can be an expression meaning it can return a value
        let x = 5;
        let y = if x == 5 { 0 } else { 100 };

        assert!(y == 0 && x == 5);
    }

    #[test]
    fn if_let_option_match_pattern() {
        let x = Some(5);
        if let Some(y) = x {
            assert!(y == 5);
        } else {
            // x is None
        }

        assert!(x != None);
    }

    /* #[test]
    fn if_let_option_match_pattern_2() {
        let x = Some(5);
        if let Some(5) = x {
            // x
        } else if let Some(4) = x {
            // x is None
        } else {
        }

        assert!(x != None);
    } */

    /*#[test]
    fn if_let_result_match_pattern() {
        let x: Result<i32, _> = Ok(5);
        if let Ok(y) = x {
            assert!(y == 5);
        } else {
            // x is Error
        }

        assert!(x.is_ok());
    }*/
}
