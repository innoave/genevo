use galvanic_assert::matchers::*;

mod timed_fn {

    use super::*;
    use crate::statistic::timed;
    use chrono;
    use std::{thread, time::Duration};

    #[test]
    fn timed_function_calls_return_a_time_greater_0() {
        let result = timed(|| {
            thread::sleep(Duration::from_millis(141));
        })
        .run();

        expect_that!(
            &result.time.duration(),
            greater_than_or_equal(chrono::Duration::milliseconds(141))
        );
        expect_that!(
            &result.time.duration(),
            less_than(chrono::Duration::milliseconds(160))
        );
    }

    #[test]
    fn timed_function_calls_measure_time_in_nanoseconds() {
        let result = timed(|| {
            thread::sleep(Duration::from_nanos(141));
        })
        .run();

        expect_that!(
            &result.time.duration(),
            greater_than_or_equal(chrono::Duration::nanoseconds(141))
        );
        expect_that!(
            &result.time.duration(),
            less_than(chrono::Duration::milliseconds(5))
        );
    }
}
