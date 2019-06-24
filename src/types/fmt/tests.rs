use super::*;
use chrono::Duration;
use galvanic_assert::matchers::*;

#[test]
fn duration_fmt_zero() {
    assert_that!(&Duration::zero().fmt(), eq("0s".to_string()));
}

#[test]
fn duration_fmt_max() {
    assert_that!(
        &Duration::max_value().fmt(),
        eq("15250284452w 3d 7h 12m 55s".to_string())
    );
}

#[test]
fn duration_fmt_min() {
    assert_that!(
        &Duration::min_value().fmt(),
        eq("-15250284452w 3d 7h 12m 55s".to_string())
    );
}

#[test]
fn duration_fmt_1ns() {
    assert_that!(&Duration::nanoseconds(1).fmt(), eq("1ns".to_string()));
}

#[test]
fn duration_fmt_1000ns() {
    assert_that!(&Duration::microseconds(1).fmt(), eq("1,000ns".to_string()));
}

#[test]
fn duration_fmt_1ms() {
    assert_that!(&Duration::milliseconds(1).fmt(), eq("1ms".to_string()));
}

#[test]
fn duration_fmt_1s() {
    assert_that!(&Duration::seconds(1).fmt(), eq("1s".to_string()));
}

#[test]
fn duration_fmt_1m() {
    assert_that!(&Duration::minutes(1).fmt(), eq("1m".to_string()));
}

#[test]
fn duration_fmt_1h() {
    assert_that!(&Duration::hours(1).fmt(), eq("1h".to_string()));
}

#[test]
fn duration_fmt_1d() {
    assert_that!(&Duration::days(1).fmt(), eq("1d".to_string()));
}

#[test]
fn duration_fmt_6d() {
    assert_that!(&Duration::days(6).fmt(), eq("6d".to_string()));
}

#[test]
fn duration_fmt_1w() {
    assert_that!(&Duration::days(7).fmt(), eq("1w".to_string()));
}

#[test]
fn duration_fmt_1s_1ms() {
    assert_that!(
        &Duration::milliseconds(1_001).fmt(),
        eq("1s 1ms".to_string())
    );
}

#[test]
fn duration_fmt_1s_1ms_1000ns() {
    assert_that!(
        &Duration::nanoseconds(1_001_001_000).fmt(),
        eq("1s 1ms 1,000ns".to_string())
    );
}

#[test]
fn duration_fmt_1s_1ms_1ns() {
    assert_that!(
        &Duration::nanoseconds(1_001_000_001).fmt(),
        eq("1s 1ms 1ns".to_string())
    );
}

#[test]
fn duration_fmt_999ms_999999ns() {
    assert_that!(
        &Duration::nanoseconds(999_999_999).fmt(),
        eq("999ms 999,999ns".to_string())
    );
}

#[test]
fn duration_fmt_1m_1s() {
    assert_that!(&Duration::seconds(61).fmt(), eq("1m 1s".to_string()));
}

#[test]
fn duration_fmt_1h_1s() {
    assert_that!(&Duration::seconds(3601).fmt(), eq("1h 0m 1s".to_string()));
}

#[test]
fn duration_fmt_1d_1s() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 1).fmt(),
        eq("1d 0h 0m 1s".to_string())
    );
}

#[test]
fn duration_fmt_1d_1h() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 3600).fmt(),
        eq("1d 1h".to_string())
    );
}

#[test]
fn duration_fmt_1d_1h_1m() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 3600 + 60).fmt(),
        eq("1d 1h 1m".to_string())
    );
}

#[test]
fn duration_fmt_1d_1h_1s() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 3600 + 1).fmt(),
        eq("1d 1h 0m 1s".to_string())
    );
}

#[test]
fn duration_fmt_1d_1m_1s() {
    assert_that!(
        &Duration::seconds(24 * 3600 + 60 + 1).fmt(),
        eq("1d 0h 1m 1s".to_string())
    );
}

#[test]
fn duration_fmt_1h_1m_1s() {
    assert_that!(
        &Duration::seconds(3600 + 60 + 1).fmt(),
        eq("1h 1m 1s".to_string())
    );
}

#[test]
fn duration_fmt_1h_0m_1s() {
    assert_that!(
        &Duration::seconds(3600 + 1).fmt(),
        eq("1h 0m 1s".to_string())
    );
}

#[test]
fn duration_fmt_1m_1ms() {
    assert_that!(
        &Duration::milliseconds(60_000 + 1).fmt(),
        eq("1m 0s 1ms".to_string())
    );
}

#[test]
fn duration_fmt_1m_1000ns() {
    assert_that!(
        &Duration::microseconds(60_000_000 + 1).fmt(),
        eq("1m 0s 0ms 1,000ns".to_string())
    );
}

#[test]
fn duration_fmt_1w_6d() {
    assert_that!(&Duration::days(13).fmt(), eq("1w 6d".to_string()));
}

#[test]
fn duration_fmt_2w() {
    assert_that!(&Duration::days(14).fmt(), eq("2w".to_string()));
}

#[test]
fn duration_fmt_2w_1d() {
    assert_that!(&Duration::days(15).fmt(), eq("2w 1d".to_string()));
}

#[test]
fn duration_fmt_1000w_6d_23h_59m_59s_999ms_999999ns() {
    let duration = 1_001 * 7 * 24 * 3600 * 1_000_000_000 - 1;
    assert_that!(
        &Duration::nanoseconds(duration).fmt(),
        eq("1000w 6d 23h 59m 59s 999ms 999,999ns".to_string())
    );
}

#[test]
fn duration_fmt_1w_999ms() {
    assert_that!(
        &Duration::milliseconds(1 * 7 * 24 * 3600 * 1_000 + 999).fmt(),
        eq("1w 0d 0h 0m 0s 999ms".to_string())
    );
}
