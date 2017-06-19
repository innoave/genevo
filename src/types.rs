//! This module provides implementations of the `Fitness` trait for
//! some primitive types, such as `i32`, `i64` et cetera.
//! This is because Rust does not allow programmers to implement
//! a foreign trait for a foreign type, which would stop you as a library user
//! from using primitive types as fitness values.
//!
//! Implemented types:
//!
//! * `i8`
//! * `i16`
//! * `i32`
//! * `i64`
//! * `u8`
//! * `u16`
//! * `u32`
//! * `u64`
//! * `usize`

use chrono::Duration;
use genetic::Fitness;

macro_rules! implement_fitness_for_signed_integer {
    ( $($t:ty),* ) => {
        $(
            impl Fitness for $t {
                fn zero() -> $t {
                    0
                }

                fn abs_diff(&self, other: &$t) -> $t {
                    let diff = self - other;
                    diff.abs()
                }
            }
        )*
    }
}

implement_fitness_for_signed_integer!(i8, i16, i32, i64, isize);


macro_rules! implement_fitness_for_unsigned_integer {
    ( $($t:ty),* ) => {
        $(
            impl Fitness for $t {
                fn zero() -> $t {
                    0
                }

                fn abs_diff(&self, other: &$t) -> $t {
                    if self > other {
                        self - other
                    } else {
                        other - self
                    }
                }
            }
        )*
    }
}

implement_fitness_for_unsigned_integer!(u8, u16, u32, u64, usize);

pub trait Display {
    fn fmt(&self) -> String;
    fn fmt_seconds(&self, always_print_till_seconds: bool) -> String;
    fn fmt_sub_seconds(&self, always_print_millis: bool) -> String;
}

impl Display for Duration {

    fn fmt(&self) -> String {
        let (sign, abs) = duration_sign_abs(self);
        let duration_secs = abs.num_seconds();
        let duration_nanos = duration_sub_seconds(self);
        if duration_secs == 0 {
            if duration_nanos == 0 {
                0.to_string() + "s"
            } else {
                sign + &fmt_duration_sub_seconds(&duration_nanos, false)
            }
        } else {
            if duration_nanos == 0 {
                sign + &fmt_duration_seconds(&duration_secs, false)
            } else {
                sign + &fmt_duration_seconds(&duration_secs, true)
                    + " " + &fmt_duration_sub_seconds(&duration_nanos, true)
            }
        }
    }

    fn fmt_seconds(&self, always_print_till_seconds: bool) -> String {
        let (sign, abs) = duration_sign_abs(self);
        sign + &fmt_duration_seconds(&abs.num_seconds(), always_print_till_seconds)
    }

    fn fmt_sub_seconds(&self, always_print_millis: bool) -> String {
        let (sign, abs) = duration_sign_abs(self);
        sign + &fmt_duration_sub_seconds(&duration_sub_seconds(&abs), always_print_millis)
    }
}

fn duration_sign_abs(duration: &Duration) -> (String, Duration) {
    if duration.num_seconds() < 0 {
        ("-".to_string(), -*duration)
    } else {
        ("".to_string(), *duration)
    }
}

fn duration_sub_seconds(duration: &Duration) -> i64 {
    match duration.num_nanoseconds() {
        Some(nanos) => nanos % 1_000_000_000,
        None => 0,
    }
}

//TODO maybe write weeks to formatted string for longer durations
//TODO do benchmarks for format! macro and pure String concatenation
fn fmt_duration_seconds(duration_secs: &i64, always_print_till_seconds: bool) -> String {
    let days = duration_secs / (24 * 60 * 60);
    let sub_days = duration_secs % (24 * 60 * 60);
    let hours = sub_days / (60 * 60);
    let sub_hours = sub_days % (60 * 60);
    let mins = sub_hours / 60;
    let secs = sub_hours % 60;
    if days == 0 {
        if hours == 0 {
            if mins == 0 {
                format!("{}s", secs)
            } else {
                if secs == 0 && !always_print_till_seconds {
                    format!("{}m", mins)
                } else {
                    format!("{}m {}s", mins, secs)
                }
            }
        } else {
            if secs == 0 && !always_print_till_seconds {
                if mins == 0 {
                    format!("{}h", hours)
                } else {
                    format!("{}h {}m", hours, mins)
                }
            } else {
                format!("{}h {}m {}s", hours, mins, secs)
            }
        }
    } else {
        if secs == 0 && !always_print_till_seconds {
            if mins == 0 {
                if hours == 0 {
                    format!("{}d", days)
                } else {
                    format!("{}d {}h", days, hours)
                }
            } else {
                format!("{}d {}h {}m", days, hours, mins)
            }
        } else {
            format!("{}d {}h {}m {}s", days, hours, mins, secs)
        }
    }
}

fn fmt_duration_sub_seconds(duration_nanos: &i64, always_print_millis: bool) -> String {
    let millis = duration_nanos / 1_000_000;
    let nanos = duration_nanos % 1_000;
    let micros = duration_nanos / 1_000 % 1_000;
    if millis == 0 && !always_print_millis {
        if nanos == 0 {
            if micros == 0 {
                String::new()
            } else {
                format!("{},000ns", micros)
            }
        } else {
            if micros == 0 {
                format!("{}ns", nanos)
            } else {
                format!("{},{:03}ns", micros, nanos)
            }
        }
    } else {
        if nanos == 0 {
            if micros == 0 {
                format!("{}ms", millis)
            } else {
                format!("{}ms {},000ns", millis, micros)
            }
        } else {
            if micros == 0 {
                format!("{}ms {}ns", millis, nanos)
            } else {
                format!("{}ms {},{:03}ns", millis, micros, nanos)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
    use std::i8;
    use std::i16;
    use std::i32;
    use std::i64;
    use std::isize;
    use std::u8;
    use std::u16;
    use std::u32;
    use std::u64;
    use std::usize;

    #[test]
    fn abs_diff_of_signed_1_and_0() {
        assert_that!(1i8.abs_diff(&0i8), is(equal_to(1i8)));
        assert_that!(1i16.abs_diff(&0i16), is(equal_to(1i16)));
        assert_that!(1i32.abs_diff(&0i32), is(equal_to(1i32)));
        assert_that!(1i64.abs_diff(&0i64), is(equal_to(1i64)));
        assert_that!(1isize.abs_diff(&0isize), is(equal_to(1isize)));
    }

    #[test]
    fn abs_diff_of_signed_0_and_1() {
        assert_that!(0i8.abs_diff(&1i8), is(equal_to(1i8)));
        assert_that!(0i16.abs_diff(&1i16), is(equal_to(1i16)));
        assert_that!(0i32.abs_diff(&1i32), is(equal_to(1i32)));
        assert_that!(0i64.abs_diff(&1i64), is(equal_to(1i64)));
        assert_that!(0isize.abs_diff(&1isize), is(equal_to(1isize)));
    }

    #[test]
    fn abs_diff_of_signed_0_and_0() {
        assert_that!(0i8.abs_diff(&0i8), is(equal_to(0i8)));
        assert_that!(0i16.abs_diff(&0i16), is(equal_to(0i16)));
        assert_that!(0i32.abs_diff(&0i32), is(equal_to(0i32)));
        assert_that!(0i64.abs_diff(&0i64), is(equal_to(0i64)));
        assert_that!(0isize.abs_diff(&0isize), is(equal_to(0isize)));
    }

    #[test]
    fn abs_diff_of_signed_neg1_and_0() {
        assert_that!((-1i8   ).abs_diff(&0i8   ), is(equal_to(1i8   )));
        assert_that!((-1i16  ).abs_diff(&0i16  ), is(equal_to(1i16  )));
        assert_that!((-1i32  ).abs_diff(&0i32  ), is(equal_to(1i32  )));
        assert_that!((-1i64  ).abs_diff(&0i64  ), is(equal_to(1i64  )));
        assert_that!((-1isize).abs_diff(&0isize), is(equal_to(1isize)));
    }

    #[test]
    fn abs_diff_of_signed_0_and_neg1() {
        assert_that!(0i8   .abs_diff(&-1i8   ), is(equal_to(1i8   )));
        assert_that!(0i16  .abs_diff(&-1i16  ), is(equal_to(1i16  )));
        assert_that!(0i32  .abs_diff(&-1i32  ), is(equal_to(1i32  )));
        assert_that!(0i64  .abs_diff(&-1i64  ), is(equal_to(1i64  )));
        assert_that!(0isize.abs_diff(&-1isize), is(equal_to(1isize)));
    }

    #[test]
    fn abs_diff_of_signed_neg1_and_1() {
        assert_that!((-1i8   ).abs_diff(&1i8   ), is(equal_to(2i8   )));
        assert_that!((-1i16  ).abs_diff(&1i16  ), is(equal_to(2i16  )));
        assert_that!((-1i32  ).abs_diff(&1i32  ), is(equal_to(2i32  )));
        assert_that!((-1i64  ).abs_diff(&1i64  ), is(equal_to(2i64  )));
        assert_that!((-1isize).abs_diff(&1isize), is(equal_to(2isize)));
    }

    #[test]
    fn abs_diff_of_signed_1_and_neg1() {
        assert_that!(1i8   .abs_diff(&-1i8   ), is(equal_to(2i8   )));
        assert_that!(1i16  .abs_diff(&-1i16  ), is(equal_to(2i16  )));
        assert_that!(1i32  .abs_diff(&-1i32  ), is(equal_to(2i32  )));
        assert_that!(1i64  .abs_diff(&-1i64  ), is(equal_to(2i64  )));
        assert_that!(1isize.abs_diff(&-1isize), is(equal_to(2isize)));
    }

    #[test]
    fn abs_diff_of_signed_neg19_and_23() {
        assert_that!((-19i8   ).abs_diff(&23i8   ), is(equal_to(42i8   )));
        assert_that!((-19i16  ).abs_diff(&23i16  ), is(equal_to(42i16  )));
        assert_that!((-19i32  ).abs_diff(&23i32  ), is(equal_to(42i32  )));
        assert_that!((-19i64  ).abs_diff(&23i64  ), is(equal_to(42i64  )));
        assert_that!((-19isize).abs_diff(&23isize), is(equal_to(42isize)));
    }

    #[test]
    fn abs_diff_of_signed_19_and_neg23() {
        assert_that!(19i8   .abs_diff(&-23i8   ), is(equal_to(42i8   )));
        assert_that!(19i16  .abs_diff(&-23i16  ), is(equal_to(42i16  )));
        assert_that!(19i32  .abs_diff(&-23i32  ), is(equal_to(42i32  )));
        assert_that!(19i64  .abs_diff(&-23i64  ), is(equal_to(42i64  )));
        assert_that!(19isize.abs_diff(&-23isize), is(equal_to(42isize)));
    }

    #[test]
    fn abs_diff_of_signed_61_and_19() {
        assert_that!(61i8.abs_diff(&19i8), is(equal_to(42i8)));
        assert_that!(61i16.abs_diff(&19i16), is(equal_to(42i16)));
        assert_that!(61i32.abs_diff(&19i32), is(equal_to(42i32)));
        assert_that!(61i64.abs_diff(&19i64), is(equal_to(42i64)));
        assert_that!(61isize.abs_diff(&19isize), is(equal_to(42isize)));
    }

    #[test]
    fn abs_diff_of_signed_19_and_61() {
        assert_that!(19i8.abs_diff(&61i8), is(equal_to(42i8)));
        assert_that!(19i16.abs_diff(&61i16), is(equal_to(42i16)));
        assert_that!(19i32.abs_diff(&61i32), is(equal_to(42i32)));
        assert_that!(19i64.abs_diff(&61i64), is(equal_to(42i64)));
        assert_that!(19isize.abs_diff(&61isize), is(equal_to(42isize)));
    }

    #[test]
    fn abs_diff_of_signed_max_and_1() {
        assert_that!(i8   ::MAX.abs_diff(&1i8   ), is(equal_to(i8   ::MAX - 1)));
        assert_that!(i16  ::MAX.abs_diff(&1i16  ), is(equal_to(i16  ::MAX - 1)));
        assert_that!(i32  ::MAX.abs_diff(&1i32  ), is(equal_to(i32  ::MAX - 1)));
        assert_that!(i64  ::MAX.abs_diff(&1i64  ), is(equal_to(i64  ::MAX - 1)));
        assert_that!(isize::MAX.abs_diff(&1isize), is(equal_to(isize::MAX - 1)));
    }

    #[test]
    fn abs_diff_of_signed_1_and_max() {
        assert_that!(1i8   .abs_diff(&i8   ::MAX), is(equal_to(i8   ::MAX - 1)));
        assert_that!(1i16  .abs_diff(&i16  ::MAX), is(equal_to(i16  ::MAX - 1)));
        assert_that!(1i32  .abs_diff(&i32  ::MAX), is(equal_to(i32  ::MAX - 1)));
        assert_that!(1i64  .abs_diff(&i64  ::MAX), is(equal_to(i64  ::MAX - 1)));
        assert_that!(1isize.abs_diff(&isize::MAX), is(equal_to(isize::MAX - 1)));
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn abs_diff_of_signed_max_and_neg1i8() {
        assert_that!(i8   ::MAX.abs_diff(&-1i8   ), is(equal_to(i8   ::MAX - 1)));
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn abs_diff_of_signed_max_and_neg1i16() {
        assert_that!(i16  ::MAX.abs_diff(&-1i16  ), is(equal_to(i16  ::MAX - 1)));
    }
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn abs_diff_of_signed_max_and_neg1i32() {
        assert_that!(i32  ::MAX.abs_diff(&-1i32  ), is(equal_to(i32  ::MAX - 1)));
    }
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn abs_diff_of_signed_max_and_neg1i64() {
        assert_that!(i64  ::MAX.abs_diff(&-1i64  ), is(equal_to(i64  ::MAX - 1)));
    }

    #[test]
    #[should_panic(expected = "attempt to negate with overflow")]
    fn abs_diff_of_signed_neg1i8_and_max() {
        assert_that!((-1i8   ).abs_diff(&i8   ::MAX), is(equal_to(i8   ::MAX)));
    }

    #[test]
    #[should_panic(expected = "attempt to negate with overflow")]
    fn abs_diff_of_signed_neg1i16_and_max() {
        assert_that!((-1i16  ).abs_diff(&i16  ::MAX), is(equal_to(i16  ::MAX)));
    }

    #[test]
    #[should_panic(expected = "attempt to negate with overflow")]
    fn abs_diff_of_signed_neg1i32_and_max() {
        assert_that!((-1i32  ).abs_diff(&i32  ::MAX), is(equal_to(i32  ::MAX)));
    }

    #[test]
    #[should_panic(expected = "attempt to negate with overflow")]
    fn abs_diff_of_signed_neg1i64_and_max() {
        assert_that!((-1i64  ).abs_diff(&i64  ::MAX), is(equal_to(i64  ::MAX)));
    }

    #[test]
    fn abs_diff_of_unsigned_1_and_0() {
        assert_that!(1u8.abs_diff(&0u8), is(equal_to(1u8)));
        assert_that!(1u16.abs_diff(&0u16), is(equal_to(1u16)));
        assert_that!(1u32.abs_diff(&0u32), is(equal_to(1u32)));
        assert_that!(1u64.abs_diff(&0u64), is(equal_to(1u64)));
        assert_that!(1usize.abs_diff(&0usize), is(equal_to(1usize)));
    }

    #[test]
    fn abs_diff_of_unsigned_0_and_1() {
        assert_that!(0u8.abs_diff(&1u8), is(equal_to(1u8)));
        assert_that!(0u16.abs_diff(&1u16), is(equal_to(1u16)));
        assert_that!(0u32.abs_diff(&1u32), is(equal_to(1u32)));
        assert_that!(0u64.abs_diff(&1u64), is(equal_to(1u64)));
        assert_that!(0usize.abs_diff(&1usize), is(equal_to(1usize)));
    }

    #[test]
    fn abs_diff_of_unsigned_0_and_0() {
        assert_that!(0u8.abs_diff(&0u8), is(equal_to(0u8)));
        assert_that!(0u16.abs_diff(&0u16), is(equal_to(0u16)));
        assert_that!(0u32.abs_diff(&0u32), is(equal_to(0u32)));
        assert_that!(0u64.abs_diff(&0u64), is(equal_to(0u64)));
        assert_that!(0usize.abs_diff(&0usize), is(equal_to(0usize)));
    }

    #[test]
    fn abs_diff_of_unsigned_61_and_19() {
        assert_that!(61u8.abs_diff(&19u8), is(equal_to(42u8)));
        assert_that!(61u16.abs_diff(&19u16), is(equal_to(42u16)));
        assert_that!(61u32.abs_diff(&19u32), is(equal_to(42u32)));
        assert_that!(61u64.abs_diff(&19u64), is(equal_to(42u64)));
        assert_that!(61usize.abs_diff(&19usize), is(equal_to(42usize)));
    }

    #[test]
    fn abs_diff_of_unsigned_19_and_61() {
        assert_that!(19u8.abs_diff(&61u8), is(equal_to(42u8)));
        assert_that!(19u16.abs_diff(&61u16), is(equal_to(42u16)));
        assert_that!(19u32.abs_diff(&61u32), is(equal_to(42u32)));
        assert_that!(19u64.abs_diff(&61u64), is(equal_to(42u64)));
        assert_that!(19usize.abs_diff(&61usize), is(equal_to(42usize)));
    }

    #[test]
    fn abs_diff_of_unsigned_max_and_1() {
        assert_that!(u8   ::MAX.abs_diff(&1u8   ), is(equal_to(u8   ::MAX - 1)));
        assert_that!(u16  ::MAX.abs_diff(&1u16  ), is(equal_to(u16  ::MAX - 1)));
        assert_that!(u32  ::MAX.abs_diff(&1u32  ), is(equal_to(u32  ::MAX - 1)));
        assert_that!(u64  ::MAX.abs_diff(&1u64  ), is(equal_to(u64  ::MAX - 1)));
        assert_that!(usize::MAX.abs_diff(&1usize), is(equal_to(usize::MAX - 1)));
    }

    #[test]
    fn abs_diff_of_unsigned_1_and_max() {
        assert_that!(1u8   .abs_diff(&u8   ::MAX), is(equal_to(u8   ::MAX - 1)));
        assert_that!(1u16  .abs_diff(&u16  ::MAX), is(equal_to(u16  ::MAX - 1)));
        assert_that!(1u32  .abs_diff(&u32  ::MAX), is(equal_to(u32  ::MAX - 1)));
        assert_that!(1u64  .abs_diff(&u64  ::MAX), is(equal_to(u64  ::MAX - 1)));
        assert_that!(1usize.abs_diff(&usize::MAX), is(equal_to(usize::MAX - 1)));
    }

    #[test]
    fn duration_fmt_zero() {
        assert_that!(&Duration::zero().fmt(), is(equal_to("0s")));
    }

    #[test]
    fn duration_fmt_max() {
        assert_that!(&Duration::max_value().fmt(), is(equal_to("106751991167d 7h 12m 55s")));
    }

    #[test]
    fn duration_fmt_min() {
        assert_that!(&Duration::min_value().fmt(), is(equal_to("-106751991167d 7h 12m 55s")));
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
    fn duration_fmt_1000d() {
        assert_that!(&Duration::days(1000).fmt(), is(equal_to("1000d")));
    }

    #[test]
    fn duration_fmt_1s_1ms() {
        assert_that!(&Duration::milliseconds(1_001).fmt(), is(equal_to("1s 1ms")));
    }

    #[test]
    fn duration_fmt_1s_1ms_1000ns() {
        assert_that!(&Duration::nanoseconds(1_001_001_000).fmt(), is(equal_to("1s 1ms 1,000ns")));
    }

    #[test]
    fn duration_fmt_1s_1ms_1ns() {
        assert_that!(&Duration::nanoseconds(1_001_000_001).fmt(), is(equal_to("1s 1ms 1ns")));
    }

    #[test]
    fn duration_fmt_999ms_999999ns() {
        assert_that!(&Duration::nanoseconds(999_999_999).fmt(), is(equal_to("999ms 999,999ns")));
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
        assert_that!(&Duration::seconds(24 * 3600 + 1).fmt(), is(equal_to("1d 0h 0m 1s")));
    }

    #[test]
    fn duration_fmt_1d_1h() {
        assert_that!(&Duration::seconds(24 * 3600 + 3600).fmt(), is(equal_to("1d 1h")));
    }

    #[test]
    fn duration_fmt_1d_1h_1m() {
        assert_that!(&Duration::seconds(24 * 3600 + 3600 + 60).fmt(), is(equal_to("1d 1h 1m")));
    }

    #[test]
    fn duration_fmt_1d_1h_1s() {
        assert_that!(&Duration::seconds(24 * 3600 + 3600 + 1).fmt(), is(equal_to("1d 1h 0m 1s")));
    }

    #[test]
    fn duration_fmt_1d_1m_1s() {
        assert_that!(&Duration::seconds(24 * 3600 + 60 + 1).fmt(), is(equal_to("1d 0h 1m 1s")));
    }

    #[test]
    fn duration_fmt_1h_1m_1s() {
        assert_that!(&Duration::seconds(3600 + 60 + 1).fmt(), is(equal_to("1h 1m 1s")));
    }

    #[test]
    fn duration_fmt_1h_0m_1s() {
        assert_that!(&Duration::seconds(3600 + 1).fmt(), is(equal_to("1h 0m 1s")));
    }

    #[test]
    fn duration_fmt_1m_1ms() {
        assert_that!(&Duration::milliseconds(60_000 + 1).fmt(), is(equal_to("1m 0s 1ms")));
    }

    #[test]
    fn duration_fmt_1m_1000ns() {
        assert_that!(&Duration::microseconds(60_000_000 + 1).fmt(), is(equal_to("1m 0s 0ms 1,000ns")));
    }

}
