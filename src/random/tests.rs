
use hamcrest::prelude::*;
use quickcheck::TestResult;
use super::*;
use random::{SeedableRng, StdRng, thread_rng};

quickcheck! {

    fn in_random_cut_points_from_range_cutpoint1_is_smaller_than_cutpoint2(
        min: usize, max: usize) -> TestResult {
        if max < min + 4 { return TestResult::discard() }

        let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

        if cutpoint1 < cutpoint2 {
            TestResult::passed()
        } else {
            TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
        }
    }

    fn in_random_cut_points_from_range_delta_of_cutpoint1_and_cutpoint2_is_smaller_than_range_minus_2(
        min: usize, max: usize) -> TestResult {
        if max < min + 4 { return TestResult::discard() }

        let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

        if cutpoint2 - cutpoint1 < max - min - 2 {
            TestResult::passed()
        } else {
            TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
        }
    }

    fn in_random_cut_points_from_range_cutpoint1_is_not_smaller_than_min_of_range(
        min: usize, max: usize) -> TestResult {
        if max < min + 4 { return TestResult::discard() }

        let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

        if cutpoint1 >= min {
            TestResult::passed()
        } else {
            TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
        }
    }

    fn in_random_cut_points_from_range_cutpoint2_is_not_greater_than_max_of_range(
        min: usize, max: usize) -> TestResult {
        if max < min + 4 { return TestResult::discard() }

        let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

        if cutpoint2 <= max {
            TestResult::passed()
        } else {
            TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
        }
    }

    fn in_random_n_cut_points_cutpoints_are_ordered_ascending(
        n: usize, length: usize) -> TestResult {
        if n == 0 { return TestResult::discard() }
        if length < 2 * n { return TestResult::discard() }

        let cutpoints = random_n_cut_points(&mut thread_rng(), n, length);

        for i in 0..cutpoints.len() - 1 {
            if cutpoints[i] == cutpoints[i + 1] {
                return TestResult::error(format!("cut points: {}:{}, {}:{} are identical",
                                                 i, cutpoints[i], i + 1, cutpoints[i + 1]));
            }
            if cutpoints[i] > cutpoints[i + 1] {
                return TestResult::error(format!("cut points: {}:{}, {}:{} are not in ascending order",
                                                 i, cutpoints[i], i + 1, cutpoints[i + 1]));
            }
        }
        TestResult::passed()
    }
}

#[test]
#[should_panic(expected = "assertion failed: max >= min + 4")]
fn random_cut_points_from_range_0_to_3() {
    random_cut_points_from_range(&mut thread_rng(), 0, 3);
}

#[test]
#[should_panic(expected = "assertion failed: max >= min + 4")]
fn random_cut_points_from_range_4_to_4() {
    random_cut_points_from_range(&mut thread_rng(), 4, 4);
}

#[test]
#[should_panic(expected = "assertion failed: n > 0")]
fn random_n_cut_points_0_4() {
    random_n_cut_points(&mut thread_rng(), 0, 4);
}

#[test]
#[should_panic(expected = "assertion failed: length >= 2 * n")]
fn random_n_cut_points_3_4() {
    random_n_cut_points(&mut thread_rng(), 3, 4);
}

#[test]
fn weighted_distribution_select() {
    let mut rng = StdRng::from_seed(&[42usize]);

    let weights = vec![200, 150, 600, 50];
    let n_sum = 1_000.;

    let weighted_distribution = WeightedDistribution::from_scalar_values(&weights);

    let mut counter = vec![0, 0, 0, 0];
    for _ in 0..n_sum as usize {
        let random = rng.next_f64() * weighted_distribution.sum();
        let index = weighted_distribution.select(random);
        counter[index] += 1;
    }

    assert_that!(counter[0], is(equal_to(204)));
    assert_that!(counter[1], is(equal_to(152)));
    assert_that!(counter[2], is(equal_to(600)));
    assert_that!(counter[3], is(equal_to(44)));
}
