#[cfg(test)]
mod loops {
    #[test]
    fn loop_basic() {
        let mut i = 0;

        loop {
            if i > 2 {
                break;
            }
            i += 1;
        }

        assert!(i == 3);
    }

    #[test]
    fn loop_can_return() {
        let mut i = 0;

        let ret = loop {
            if i > 2 {
                break i * 2;
            }
            i += 1;
        };

        assert!(i == 3 && ret == 6);
    }

    #[test]
    fn while_basic() {
        let mut i = 0;

        while i < 3 {
            i += 1;
        }

        assert!(i == 3);
    }

    #[test]
    #[allow(while_true)]
    fn while_forever() {
        let mut i = 0;

        while true {
            if i > 2 {
                break;
            }
            i += 1;
        }

        assert!(i == 3);
    }
}
