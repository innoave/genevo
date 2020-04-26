use super::*;
use galvanic_assert::matchers::*;
use std::{i16, i32, i64, i8, isize, u16, u32, u64, u8, usize};

#[test]
fn abs_diff_of_signed_1_and_0() {
    expect_that!(&1i8.abs_diff(&0i8), is(equal_to(1i8)));
    expect_that!(&1i16.abs_diff(&0i16), is(equal_to(1i16)));
    expect_that!(&1i32.abs_diff(&0i32), is(equal_to(1i32)));
    expect_that!(&1i64.abs_diff(&0i64), is(equal_to(1i64)));
    expect_that!(&1isize.abs_diff(&0isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_0_and_1() {
    expect_that!(&0i8.abs_diff(&1i8), is(equal_to(1i8)));
    expect_that!(&0i16.abs_diff(&1i16), is(equal_to(1i16)));
    expect_that!(&0i32.abs_diff(&1i32), is(equal_to(1i32)));
    expect_that!(&0i64.abs_diff(&1i64), is(equal_to(1i64)));
    expect_that!(&0isize.abs_diff(&1isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_0_and_0() {
    expect_that!(&0i8.abs_diff(&0i8), is(equal_to(0i8)));
    expect_that!(&0i16.abs_diff(&0i16), is(equal_to(0i16)));
    expect_that!(&0i32.abs_diff(&0i32), is(equal_to(0i32)));
    expect_that!(&0i64.abs_diff(&0i64), is(equal_to(0i64)));
    expect_that!(&0isize.abs_diff(&0isize), is(equal_to(0isize)));
}

#[test]
fn abs_diff_of_signed_neg1_and_0() {
    expect_that!(&(-1i8).abs_diff(&0i8), is(equal_to(1i8)));
    expect_that!(&(-1i16).abs_diff(&0i16), is(equal_to(1i16)));
    expect_that!(&(-1i32).abs_diff(&0i32), is(equal_to(1i32)));
    expect_that!(&(-1i64).abs_diff(&0i64), is(equal_to(1i64)));
    expect_that!(&(-1isize).abs_diff(&0isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_0_and_neg1() {
    expect_that!(&0i8.abs_diff(&-1i8), is(equal_to(1i8)));
    expect_that!(&0i16.abs_diff(&-1i16), is(equal_to(1i16)));
    expect_that!(&0i32.abs_diff(&-1i32), is(equal_to(1i32)));
    expect_that!(&0i64.abs_diff(&-1i64), is(equal_to(1i64)));
    expect_that!(&0isize.abs_diff(&-1isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_neg1_and_1() {
    expect_that!(&(-1i8).abs_diff(&1i8), is(equal_to(2i8)));
    expect_that!(&(-1i16).abs_diff(&1i16), is(equal_to(2i16)));
    expect_that!(&(-1i32).abs_diff(&1i32), is(equal_to(2i32)));
    expect_that!(&(-1i64).abs_diff(&1i64), is(equal_to(2i64)));
    expect_that!(&(-1isize).abs_diff(&1isize), is(equal_to(2isize)));
}

#[test]
fn abs_diff_of_signed_1_and_neg1() {
    expect_that!(&1i8.abs_diff(&-1i8), is(equal_to(2i8)));
    expect_that!(&1i16.abs_diff(&-1i16), is(equal_to(2i16)));
    expect_that!(&1i32.abs_diff(&-1i32), is(equal_to(2i32)));
    expect_that!(&1i64.abs_diff(&-1i64), is(equal_to(2i64)));
    expect_that!(&1isize.abs_diff(&-1isize), is(equal_to(2isize)));
}

#[test]
fn abs_diff_of_signed_neg19_and_23() {
    expect_that!(&(-19i8).abs_diff(&23i8), is(equal_to(42i8)));
    expect_that!(&(-19i16).abs_diff(&23i16), is(equal_to(42i16)));
    expect_that!(&(-19i32).abs_diff(&23i32), is(equal_to(42i32)));
    expect_that!(&(-19i64).abs_diff(&23i64), is(equal_to(42i64)));
    expect_that!(&(-19isize).abs_diff(&23isize), is(equal_to(42isize)));
}

#[test]
fn abs_diff_of_signed_19_and_neg23() {
    expect_that!(&19i8.abs_diff(&-23i8), is(equal_to(42i8)));
    expect_that!(&19i16.abs_diff(&-23i16), is(equal_to(42i16)));
    expect_that!(&19i32.abs_diff(&-23i32), is(equal_to(42i32)));
    expect_that!(&19i64.abs_diff(&-23i64), is(equal_to(42i64)));
    expect_that!(&19isize.abs_diff(&-23isize), is(equal_to(42isize)));
}

#[test]
fn abs_diff_of_signed_61_and_19() {
    expect_that!(&61i8.abs_diff(&19i8), is(equal_to(42i8)));
    expect_that!(&61i16.abs_diff(&19i16), is(equal_to(42i16)));
    expect_that!(&61i32.abs_diff(&19i32), is(equal_to(42i32)));
    expect_that!(&61i64.abs_diff(&19i64), is(equal_to(42i64)));
    expect_that!(&61isize.abs_diff(&19isize), is(equal_to(42isize)));
}

#[test]
fn abs_diff_of_signed_19_and_61() {
    expect_that!(&19i8.abs_diff(&61i8), is(equal_to(42i8)));
    expect_that!(&19i16.abs_diff(&61i16), is(equal_to(42i16)));
    expect_that!(&19i32.abs_diff(&61i32), is(equal_to(42i32)));
    expect_that!(&19i64.abs_diff(&61i64), is(equal_to(42i64)));
    expect_that!(&19isize.abs_diff(&61isize), is(equal_to(42isize)));
}

#[test]
fn abs_diff_of_signed_max_and_1() {
    expect_that!(&i8::MAX.abs_diff(&1i8), is(equal_to(i8::MAX - 1)));
    expect_that!(&i16::MAX.abs_diff(&1i16), is(equal_to(i16::MAX - 1)));
    expect_that!(&i32::MAX.abs_diff(&1i32), is(equal_to(i32::MAX - 1)));
    expect_that!(&i64::MAX.abs_diff(&1i64), is(equal_to(i64::MAX - 1)));
    expect_that!(&isize::MAX.abs_diff(&1isize), is(equal_to(isize::MAX - 1)));
}

#[test]
fn abs_diff_of_signed_1_and_max() {
    expect_that!(&1i8.abs_diff(&i8::MAX), is(equal_to(i8::MAX - 1)));
    expect_that!(&1i16.abs_diff(&i16::MAX), is(equal_to(i16::MAX - 1)));
    expect_that!(&1i32.abs_diff(&i32::MAX), is(equal_to(i32::MAX - 1)));
    expect_that!(&1i64.abs_diff(&i64::MAX), is(equal_to(i64::MAX - 1)));
    expect_that!(&1isize.abs_diff(&isize::MAX), is(equal_to(isize::MAX - 1)));
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i8() {
    expect_that!(&i8::MAX.abs_diff(&-1i8), is(equal_to(i8::MAX - 1)));
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i16() {
    expect_that!(&i16::MAX.abs_diff(&-1i16), is(equal_to(i16::MAX - 1)));
}
#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i32() {
    expect_that!(&i32::MAX.abs_diff(&-1i32), is(equal_to(i32::MAX - 1)));
}
#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i64() {
    expect_that!(&i64::MAX.abs_diff(&-1i64), is(equal_to(i64::MAX - 1)));
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i8_and_max() {
    expect_that!(&(-1i8).abs_diff(&i8::MAX), is(equal_to(i8::MAX)));
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i16_and_max() {
    expect_that!(&(-1i16).abs_diff(&i16::MAX), is(equal_to(i16::MAX)));
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i32_and_max() {
    expect_that!(&(-1i32).abs_diff(&i32::MAX), is(equal_to(i32::MAX)));
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i64_and_max() {
    expect_that!(&(-1i64).abs_diff(&i64::MAX), is(equal_to(i64::MAX)));
}

#[test]
fn abs_diff_of_unsigned_1_and_0() {
    expect_that!(&1u8.abs_diff(&0u8), is(equal_to(1u8)));
    expect_that!(&1u16.abs_diff(&0u16), is(equal_to(1u16)));
    expect_that!(&1u32.abs_diff(&0u32), is(equal_to(1u32)));
    expect_that!(&1u64.abs_diff(&0u64), is(equal_to(1u64)));
    expect_that!(&1usize.abs_diff(&0usize), is(equal_to(1usize)));
}

#[test]
fn abs_diff_of_unsigned_0_and_1() {
    expect_that!(&0u8.abs_diff(&1u8), is(equal_to(1u8)));
    expect_that!(&0u16.abs_diff(&1u16), is(equal_to(1u16)));
    expect_that!(&0u32.abs_diff(&1u32), is(equal_to(1u32)));
    expect_that!(&0u64.abs_diff(&1u64), is(equal_to(1u64)));
    expect_that!(&0usize.abs_diff(&1usize), is(equal_to(1usize)));
}

#[test]
fn abs_diff_of_unsigned_0_and_0() {
    expect_that!(&0u8.abs_diff(&0u8), is(equal_to(0u8)));
    expect_that!(&0u16.abs_diff(&0u16), is(equal_to(0u16)));
    expect_that!(&0u32.abs_diff(&0u32), is(equal_to(0u32)));
    expect_that!(&0u64.abs_diff(&0u64), is(equal_to(0u64)));
    expect_that!(&0usize.abs_diff(&0usize), is(equal_to(0usize)));
}

#[test]
fn abs_diff_of_unsigned_61_and_19() {
    expect_that!(&61u8.abs_diff(&19u8), is(equal_to(42u8)));
    expect_that!(&61u16.abs_diff(&19u16), is(equal_to(42u16)));
    expect_that!(&61u32.abs_diff(&19u32), is(equal_to(42u32)));
    expect_that!(&61u64.abs_diff(&19u64), is(equal_to(42u64)));
    expect_that!(&61usize.abs_diff(&19usize), is(equal_to(42usize)));
}

#[test]
fn abs_diff_of_unsigned_19_and_61() {
    expect_that!(&19u8.abs_diff(&61u8), is(equal_to(42u8)));
    expect_that!(&19u16.abs_diff(&61u16), is(equal_to(42u16)));
    expect_that!(&19u32.abs_diff(&61u32), is(equal_to(42u32)));
    expect_that!(&19u64.abs_diff(&61u64), is(equal_to(42u64)));
    expect_that!(&19usize.abs_diff(&61usize), is(equal_to(42usize)));
}

#[test]
fn abs_diff_of_unsigned_max_and_1() {
    expect_that!(&u8::MAX.abs_diff(&1u8), is(equal_to(u8::MAX - 1)));
    expect_that!(&u16::MAX.abs_diff(&1u16), is(equal_to(u16::MAX - 1)));
    expect_that!(&u32::MAX.abs_diff(&1u32), is(equal_to(u32::MAX - 1)));
    expect_that!(&u64::MAX.abs_diff(&1u64), is(equal_to(u64::MAX - 1)));
    expect_that!(&usize::MAX.abs_diff(&1usize), is(equal_to(usize::MAX - 1)));
}

#[test]
fn abs_diff_of_unsigned_1_and_max() {
    expect_that!(&1u8.abs_diff(&u8::MAX), is(equal_to(u8::MAX - 1)));
    expect_that!(&1u16.abs_diff(&u16::MAX), is(equal_to(u16::MAX - 1)));
    expect_that!(&1u32.abs_diff(&u32::MAX), is(equal_to(u32::MAX - 1)));
    expect_that!(&1u64.abs_diff(&u64::MAX), is(equal_to(u64::MAX - 1)));
    expect_that!(&1usize.abs_diff(&usize::MAX), is(equal_to(usize::MAX - 1)));
}
