use super::*;
use galvanic_assert::matchers::*;
use proptest::prelude::*;

mod random_cut_points_from_range {

    use super::*;

    #[test]
    #[should_panic(expected = "assertion failed: max >= min + 4")]
    fn random_cut_points_from_range_0_to_3() {
        random_cut_points_from_range(&mut get_rng(random_seed()), 0, 3);
    }

    #[test]
    #[should_panic(expected = "assertion failed: max >= min + 4")]
    fn random_cut_points_from_range_4_to_4() {
        random_cut_points_from_range(&mut get_rng(random_seed()), 4, 4);
    }

    proptest! {

        #[test]
        fn in_random_cut_points_from_range_cutpoint1_is_smaller_than_cutpoint2(
            (min, max) in (1usize..999_999).prop_flat_map(|min|
                (Just(min), (min + 4..999_999 + 4))
            ),
        ) {
            let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut get_rng(random_seed()), min, max);

            prop_assert!(
                cutpoint1 < cutpoint2,
                "cut point 1 is less than cut point 2: {}, {}",
                cutpoint1,
                cutpoint2,
            );
        }

        #[test]
        fn in_random_cut_points_from_range_delta_between_cutpoints_is_smaller_than_range_minus_2(
            (min, max) in (1usize..999_999).prop_flat_map(|min|
                (Just(min), (min + 4..999_999 + 4))
            ),
        ) {
            let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut get_rng(random_seed()), min, max);

            prop_assert!(
                cutpoint2 - cutpoint1 < max - min - 2,
                "delta between cut points is smaller than range minus 2: {}, {}",
                cutpoint1,
                cutpoint2,
            );
        }

        #[test]
        fn in_random_cut_points_from_range_cutpoint1_is_not_smaller_than_min_of_range(
            (min, max) in (1usize..999_999).prop_flat_map(|min|
                (Just(min), (min + 4..999_999 + 4))
            ),
        ) {
            let (cutpoint1, _cutpoint2) = random_cut_points_from_range(&mut get_rng(random_seed()), min, max);

            prop_assert!(
                cutpoint1 >= min,
                "cut point 1 is not smaller than min of range: cutpoint1={}, min={}",
                 cutpoint1,
                 min,
            );
        }

        #[test]
        fn in_random_cut_points_from_range_cutpoint2_is_not_greater_than_max_of_range(
            (min, max) in (1usize..999_999 / 4).prop_flat_map(|min|
                (Just(min), (min + 4..999_999 + 4))
            ),
        ) {
            let (_cutpoint1, cutpoint2) = random_cut_points_from_range(&mut get_rng(random_seed()), min, max);

            prop_assert!(
                cutpoint2 <= max,
                "cut point 2 is not greater than max of range: cutpoint2={}, max={}",
                cutpoint2,
                max,
            );
        }
    }
}

mod random_n_cut_points {
    use super::*;

    #[test]
    #[should_panic(expected = "assertion failed: n > 0")]
    fn random_n_cut_points_0_4() {
        random_n_cut_points(&mut get_rng(random_seed()), 0, 4);
    }

    #[test]
    #[should_panic(expected = "assertion failed: length >= 2 * n")]
    fn random_n_cut_points_3_4() {
        random_n_cut_points(&mut get_rng(random_seed()), 3, 4);
    }

    proptest! {

        #[test]
        fn in_random_n_cut_points_cutpoints_are_ordered_ascending(
            (n, length) in (1usize..9_999 / 2).prop_flat_map(|n| (Just(n), (2 * n..9_999))),
        ) {
            let cutpoints = random_n_cut_points(&mut get_rng(random_seed()), n, length);

            for i in 0..cutpoints.len() - 1 {
                if cutpoints[i] == cutpoints[i + 1] {
                    prop_assert!(
                        false,
                        "cut points: {}:{}, {}:{} are identical",
                        i, cutpoints[i], i + 1, cutpoints[i + 1],
                    );
                }
                if cutpoints[i] > cutpoints[i + 1] {
                    prop_assert!(
                        false,
                        "cut points: {}:{}, {}:{} are not in ascending order",
                        i, cutpoints[i], i + 1, cutpoints[i + 1],
                    );
                }
            }
        }
    }
}

mod weighted_distribution {

    use super::*;

    #[test]
    fn weighted_distribution_select() {
        let mut rng = Prng::from_seed([42; 32]);

        let weights = vec![200, 150, 600, 50];
        let n_sum = 1_000;

        let weighted_distribution = WeightedDistribution::from_scalar_values(&weights);

        let mut counter = vec![0, 0, 0, 0];
        for _ in 0..n_sum {
            let random = rng.gen::<f64>() * weighted_distribution.sum();
            let index = weighted_distribution.select(random);
            counter[index] += 1;
        }

        expect_that!(&counter[0], is(greater_than(180)));
        expect_that!(&counter[0], is(less_than(220)));
        expect_that!(&counter[1], is(greater_than(130)));
        expect_that!(&counter[1], is(less_than(175)));
        expect_that!(&counter[2], is(greater_than(540)));
        expect_that!(&counter[2], is(less_than(660)));
        expect_that!(&counter[3], is(greater_than(40)));
        expect_that!(&counter[3], is(less_than(60)));
    }
}
