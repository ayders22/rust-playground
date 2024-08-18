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
    fn loop_with_labels() {
        let mut outer_count = 0;
        let mut inter_count = 0;
        let mut inner_count = 0;

        'outer: loop {
            outer_count += 1;

            'inter: loop {
                inter_count += 1;

                'inner: loop {
                    inner_count += 1;

                    if outer_count == 2 {
                        println!(
                            "breaking from outer loop (outer: {:?}, inter: {:?}, inner: {:?})",
                            outer_count, inter_count, inner_count
                        );
                        break 'outer;
                    }

                    if inter_count == 3 {
                        println!(
                            "breaking from inter loop (outer: {:?}, inter: {:?}, inner: {:?})",
                            outer_count, inter_count, inner_count
                        );
                        break 'inter;
                    }

                    if inner_count >= 3 {
                        println!(
                            "breaking from inner loop (outer: {:?}, inter: {:?}, inner: {:?})",
                            outer_count, inter_count, inner_count
                        );
                        break 'inner; // could have been only `break``
                    }
                }

                inner_count = 0;
            }
        }

        assert_eq!(outer_count, 2);
        assert_eq!(inter_count, 4);
        assert_eq!(inner_count, 2);
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

    #[test]
    #[allow(while_true)]
    fn while_let() {
        let mut x = Some(4);

        while let Some(i) = x {
            if i > 4 {
                x = None;
            } else {
                x = Some(i + 1)
            }
        }

        assert!(x == None);
    }

    #[test]
    #[allow(while_true)]
    fn for_basic() {
        let mut i = 0;

        for j in 0..4 {
            i += j;
        }
        // 0 + 1 + 2 + 3

        assert!(i == 6);
    }

    #[test]
    #[allow(while_true)]
    fn for_custom_step() {
        let mut i = 0;

        for j in (0..10).step_by(2) {
            i += j;
        }
        // 0 + 2 + 4 + 6 + 8

        assert!(i == 20);
    }

    #[test]
    #[allow(while_true)]
    fn for_with_enumerate() {
        let mut idx_total = 0;
        let mut cnt_total = 0;

        for (idx, cnt) in (0..10).step_by(3).enumerate() {
            idx_total += idx;
            cnt_total += cnt;
        }
        // idx_total = 0 + 1 + 2 + 3
        // cnt_total = 0 + 3 + 6 + 9

        assert!(idx_total == 6 && cnt_total == 18);
    }

    #[test]
    #[allow(while_true)]
    fn for_with_iterator() {
        let v = vec![8, 0, 9];
        let mut i = 0;

        for el in v.iter() {
            i += el;
        }
        // 8 + 0 + 9

        assert!(i == 17);
    }
}
