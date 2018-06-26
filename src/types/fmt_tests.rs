use hamcrest::prelude::*;

use super::fmt::*;
use chrono::Duration;

#[test]
fn duration_fmt_zero() {
    assert_that!(&Duration::zero().fmt(), is(equal_to("0s")));
}

#[test]
fn duration_fmt_max() {
    assert_that!(
        &Duration::max_value().fmt(),
        is(equal_to("15250284452w 3d 7h 12m 55s"))
    );
}

#[test]
fn duration_fmt_min() {
    assert_that!(
        &Duration::min_value().fmt(),
        is(equal_to("-15250284452w 3d 7h 12m 55s"))
    );
}

#[test]
fn duration_fmt_1ns() {
    assert_that!(&Duration::nanoseconds(1).fmt(), is(equal_to("1ns")));
}

#[test]
fn duration_fmt_1000ns() {
    assert_that!(&Duration::microseconds(1).fmt(), is(equal_to("1,000ns")));
}

#[test]
fn duration_fmt_1ms() {
    assert_that!(&Duration::milliseconds(1).fmt(), is(equal_to("1ms")));
}

#[test]
fn duration_fmt_1s() {
    assert_that!(&Duration::seconds(1).fmt(), is(equal_to("1s")));
}

#[test]
fn duration_fmt_1m() {
    assert_that!(&Duration::minutes(1).fmt(), is(equal_to("1m")));
}

#[test]
fn duration_fmt_1h() {
    assert_that!(&Duration::hours(1).fmt(), is(equal_to("1h")));
}

#[test]
fn duration_fmt_1d() {
    assert_that!(&Duration::days(1).fmt(), is(equal_to("1d")));
}

#[test]
fn duration_fmt_6d() {
    assert_that!(&Duration::days(6).fmt(), is(equal_to("6d")));
}

#[test]
fn duration_fmt_1w() {
    assert_that!(&Duration::days(7).fmt(), is(equal_to("1w")));
}

#[test]
fn duration_fmt_1s_1ms() {
    assert_that!(&Duration::milliseconds(1_001).fmt(), is(equal_to("1s 1ms")));
}

#[test]
fn duration_fmt_1s_1ms_1000ns() {
    assert_that!(
        &Duration::nanoseconds(1_001_001_000).fmt(),
        is(equal_to("1s 1ms 1,000ns"))
    );
}

#[test]
fn duration_fmt_1s_1ms_1ns() {
    assert_that!(
        &Duration::nanoseconds(1_001_000_001).fmt(),
        is(equal_to("1s 1ms 1ns"))
    );
}

#[test]
fn duration_fmt_999ms_999999ns() {
    assert_that!(
        &Duration::nanoseconds(999_999_999).fmt(),
        is(equal_to("999ms 999,999ns"))
    );
}

#[test]
fn duration_fmt_1m_1s() {
    assert_that!(&Duration::seconds(61).fmt(), is(equal_to("1m 1s")));
}

#[test]
fn duration_fmt_1h_1s() {
    assert_that!(&Duration::seconds(3601).fmt(), is(equal_to("1h 0m 1s")));
}

#[test]
fn duration_fmt_1d_1s() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 1).fmt(),
        is(equal_to("1d 0h 0m 1s"))
    );
}

#[test]
fn duration_fmt_1d_1h() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 3600).fmt(),
        is(equal_to("1d 1h"))
    );
}

#[test]
fn duration_fmt_1d_1h_1m() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 3600 + 60).fmt(),
        is(equal_to("1d 1h 1m"))
    );
}

#[test]
fn duration_fmt_1d_1h_1s() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 3600 + 1).fmt(),
        is(equal_to("1d 1h 0m 1s"))
    );
}

#[test]
fn duration_fmt_1d_1m_1s() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 60 + 1).fmt(),
        is(equal_to("1d 0h 1m 1s"))
    );
}

#[test]
fn duration_fmt_1h_1m_1s() {
    assert_that!(
        &Duration::seconds(3600 + 60 + 1).fmt(),
        is(equal_to("1h 1m 1s"))
    );
}

#[test]
fn duration_fmt_1h_0m_1s() {
    assert_that!(&Duration::seconds(3600 + 1).fmt(), is(equal_to("1h 0m 1s")));
}

#[test]
fn duration_fmt_1m_1ms() {
    assert_that!(
        &Duration::milliseconds(60_000 + 1).fmt(),
        is(equal_to("1m 0s 1ms"))
    );
}

#[test]
fn duration_fmt_1m_1000ns() {
    assert_that!(
        &Duration::microseconds(60_000_000 + 1).fmt(),
        is(equal_to("1m 0s 0ms 1,000ns"))
    );
}

#[test]
fn duration_fmt_1w_6d() {
    assert_that!(&Duration::days(13).fmt(), is(equal_to("1w 6d")));
}

#[test]
fn duration_fmt_2w() {
    assert_that!(&Duration::days(14).fmt(), is(equal_to("2w")));
}

#[test]
fn duration_fmt_2w_1d() {
    assert_that!(&Duration::days(15).fmt(), is(equal_to("2w 1d")));
}

#[test]
fn duration_fmt_1000w_6d_23h_59m_59s_999ms_999999ns() {
    let duration = 1_001 * 7 * 24 * 3600 * 1_000_000_000 - 1;
    assert_that!(
        &Duration::nanoseconds(duration).fmt(),
        is(equal_to("1000w 6d 23h 59m 59s 999ms 999,999ns"))
    );
}

#[test]
fn duration_fmt_1w_999ms() {
    assert_that!(
        &Duration::milliseconds(1 * 7 * 24 * 3600 * 1_000 + 999).fmt(),
        is(equal_to("1w 0d 0h 0m 0s 999ms"))
    );
}
