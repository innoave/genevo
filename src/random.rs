
use rand::Rng;


pub fn random_index<R>(rng: &mut R, length: usize) -> usize
    where R: Rng + Sized {
    random_index_from_range(rng, 0, length)
}

pub fn random_index_from_range<R>(rng: &mut R, min: usize, max: usize) -> usize
    where R: Rng + Sized {
    rng.gen_range(min, max)
}

pub fn random_cut_points<R>(rng: &mut R, length: usize) -> (usize, usize)
    where R: Rng + Sized {
    random_cut_points_from_range(rng, 0, length)
}

pub fn random_cut_points_from_range<R>(rng: &mut R, min: usize, max: usize) -> (usize, usize)
    where R: Rng + Sized {
    assert!(max >= min + 4);
    let max_slice = max - min - 2;
    loop {
        let cutpoint1 = rng.gen_range(min, max);
        let cutpoint2 = rng.gen_range(min, max);
        if cutpoint1 < cutpoint2 {
            if cutpoint2 - cutpoint1 >= max_slice {
                continue;
            }
            return (cutpoint1, cutpoint2)
        } else if cutpoint2 < cutpoint1 {
            if cutpoint1 - cutpoint2 >= max_slice {
                continue;
            }
            return (cutpoint2, cutpoint1)
        }
    }
}

pub fn random_probability<R>(rng: &mut R) -> f64
    where R: Rng + Sized {
    rng.next_f64()
}


#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;
    use rand::thread_rng;

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

    }

}
