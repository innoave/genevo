use super::*;
use galvanic_assert::matchers::*;
use std::{i16, i32, i64, i8, isize, u16, u32, u64, u8, usize};

#[test]
fn abs_diff_of_signed_1_and_0() {
    expect_that!(&Fitness::abs_diff(&1i8, &0i8), is(equal_to(1i8)));
    expect_that!(&Fitness::abs_diff(&1i16, &0i16), is(equal_to(1i16)));
    expect_that!(&Fitness::abs_diff(&1i32, &0i32), is(equal_to(1i32)));
    expect_that!(&Fitness::abs_diff(&1i64, &0i64), is(equal_to(1i64)));
    expect_that!(&Fitness::abs_diff(&1isize, &0isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_0_and_1() {
    expect_that!(&Fitness::abs_diff(&0i8, &1i8), is(equal_to(1i8)));
    expect_that!(&Fitness::abs_diff(&0i16, &1i16), is(equal_to(1i16)));
    expect_that!(&Fitness::abs_diff(&0i32, &1i32), is(equal_to(1i32)));
    expect_that!(&Fitness::abs_diff(&0i64, &1i64), is(equal_to(1i64)));
    expect_that!(&Fitness::abs_diff(&0isize, &1isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_0_and_0() {
    expect_that!(&Fitness::abs_diff(&0i8, &0i8), is(equal_to(0i8)));
    expect_that!(&Fitness::abs_diff(&0i16, &0i16), is(equal_to(0i16)));
    expect_that!(&Fitness::abs_diff(&0i32, &0i32), is(equal_to(0i32)));
    expect_that!(&Fitness::abs_diff(&0i64, &0i64), is(equal_to(0i64)));
    expect_that!(&Fitness::abs_diff(&0isize, &0isize), is(equal_to(0isize)));
}

#[test]
fn abs_diff_of_signed_neg1_and_0() {
    expect_that!(&Fitness::abs_diff(&-1i8, &0i8), is(equal_to(1i8)));
    expect_that!(&Fitness::abs_diff(&-1i16, &0i16), is(equal_to(1i16)));
    expect_that!(&Fitness::abs_diff(&-1i32, &0i32), is(equal_to(1i32)));
    expect_that!(&Fitness::abs_diff(&-1i64, &0i64), is(equal_to(1i64)));
    expect_that!(&Fitness::abs_diff(&-1isize, &0isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_0_and_neg1() {
    expect_that!(&Fitness::abs_diff(&0i8, &-1i8), is(equal_to(1i8)));
    expect_that!(&Fitness::abs_diff(&0i16, &-1i16), is(equal_to(1i16)));
    expect_that!(&Fitness::abs_diff(&0i32, &-1i32), is(equal_to(1i32)));
    expect_that!(&Fitness::abs_diff(&0i64, &-1i64), is(equal_to(1i64)));
    expect_that!(&Fitness::abs_diff(&0isize, &-1isize), is(equal_to(1isize)));
}

#[test]
fn abs_diff_of_signed_neg1_and_1() {
    expect_that!(&Fitness::abs_diff(&-1i8, &1i8), is(equal_to(2i8)));
    expect_that!(&Fitness::abs_diff(&-1i16, &1i16), is(equal_to(2i16)));
    expect_that!(&Fitness::abs_diff(&-1i32, &1i32), is(equal_to(2i32)));
    expect_that!(&Fitness::abs_diff(&-1i64, &1i64), is(equal_to(2i64)));
    expect_that!(&Fitness::abs_diff(&-1isize, &1isize), is(equal_to(2isize)));
}

#[test]
fn abs_diff_of_signed_1_and_neg1() {
    expect_that!(&Fitness::abs_diff(&1i8, &-1i8), is(equal_to(2i8)));
    expect_that!(&Fitness::abs_diff(&1i16, &-1i16), is(equal_to(2i16)));
    expect_that!(&Fitness::abs_diff(&1i32, &-1i32), is(equal_to(2i32)));
    expect_that!(&Fitness::abs_diff(&1i64, &-1i64), is(equal_to(2i64)));
    expect_that!(&Fitness::abs_diff(&1isize, &-1isize), is(equal_to(2isize)));
}

#[test]
fn abs_diff_of_signed_neg19_and_23() {
    expect_that!(&Fitness::abs_diff(&-19i8, &23i8), is(equal_to(42i8)));
    expect_that!(&Fitness::abs_diff(&-19i16, &23i16), is(equal_to(42i16)));
    expect_that!(&Fitness::abs_diff(&-19i32, &23i32), is(equal_to(42i32)));
    expect_that!(&Fitness::abs_diff(&-19i64, &23i64), is(equal_to(42i64)));
    expect_that!(
        &Fitness::abs_diff(&-19isize, &23isize),
        is(equal_to(42isize))
    );
}

#[test]
fn abs_diff_of_signed_19_and_neg23() {
    expect_that!(&Fitness::abs_diff(&19i8, &-23i8), is(equal_to(42i8)));
    expect_that!(&Fitness::abs_diff(&19i16, &-23i16), is(equal_to(42i16)));
    expect_that!(&Fitness::abs_diff(&19i32, &-23i32), is(equal_to(42i32)));
    expect_that!(&Fitness::abs_diff(&19i64, &-23i64), is(equal_to(42i64)));
    expect_that!(
        &Fitness::abs_diff(&19isize, &-23isize),
        is(equal_to(42isize))
    );
}

#[test]
fn abs_diff_of_signed_61_and_19() {
    expect_that!(&Fitness::abs_diff(&61i8, &19i8), is(equal_to(42i8)));
    expect_that!(&Fitness::abs_diff(&61i16, &19i16), is(equal_to(42i16)));
    expect_that!(&Fitness::abs_diff(&61i32, &19i32), is(equal_to(42i32)));
    expect_that!(&Fitness::abs_diff(&61i64, &19i64), is(equal_to(42i64)));
    expect_that!(
        &Fitness::abs_diff(&61isize, &19isize),
        is(equal_to(42isize))
    );
}

#[test]
fn abs_diff_of_signed_19_and_61() {
    expect_that!(&Fitness::abs_diff(&19i8, &61i8), is(equal_to(42i8)));
    expect_that!(&Fitness::abs_diff(&19i16, &61i16), is(equal_to(42i16)));
    expect_that!(&Fitness::abs_diff(&19i32, &61i32), is(equal_to(42i32)));
    expect_that!(&Fitness::abs_diff(&19i64, &61i64), is(equal_to(42i64)));
    expect_that!(
        &Fitness::abs_diff(&19isize, &61isize),
        is(equal_to(42isize))
    );
}

#[test]
fn abs_diff_of_signed_max_and_1() {
    expect_that!(
        &Fitness::abs_diff(&i8::MAX, &1i8),
        is(equal_to(i8::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&i16::MAX, &1i16),
        is(equal_to(i16::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&i32::MAX, &1i32),
        is(equal_to(i32::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&i64::MAX, &1i64),
        is(equal_to(i64::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&isize::MAX, &1isize),
        is(equal_to(isize::MAX - 1))
    );
}

#[test]
fn abs_diff_of_signed_1_and_max() {
    expect_that!(
        &Fitness::abs_diff(&1i8, &i8::MAX),
        is(equal_to(i8::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1i16, &i16::MAX),
        is(equal_to(i16::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1i32, &i32::MAX),
        is(equal_to(i32::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1i64, &i64::MAX),
        is(equal_to(i64::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1isize, &isize::MAX),
        is(equal_to(isize::MAX - 1))
    );
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i8() {
    expect_that!(
        &Fitness::abs_diff(&i8::MAX, &-1i8),
        is(equal_to(i8::MAX - 1))
    );
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i16() {
    expect_that!(
        &Fitness::abs_diff(&i16::MAX, &-1i16),
        is(equal_to(i16::MAX - 1))
    );
}
#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i32() {
    expect_that!(
        &Fitness::abs_diff(&i32::MAX, &-1i32),
        is(equal_to(i32::MAX - 1))
    );
}
#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn abs_diff_of_signed_max_and_neg1i64() {
    expect_that!(
        &Fitness::abs_diff(&i64::MAX, &-1i64),
        is(equal_to(i64::MAX - 1))
    );
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i8_and_max() {
    expect_that!(&Fitness::abs_diff(&-1i8, &i8::MAX), is(equal_to(i8::MAX)));
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i16_and_max() {
    expect_that!(
        &Fitness::abs_diff(&-1i16, &i16::MAX),
        is(equal_to(i16::MAX))
    );
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i32_and_max() {
    expect_that!(
        &Fitness::abs_diff(&-1i32, &i32::MAX),
        is(equal_to(i32::MAX))
    );
}

#[test]
#[should_panic(expected = "attempt to negate with overflow")]
fn abs_diff_of_signed_neg1i64_and_max() {
    expect_that!(
        &Fitness::abs_diff(&-1i64, &i64::MAX),
        is(equal_to(i64::MAX))
    );
}

#[test]
fn abs_diff_of_unsigned_1_and_0() {
    expect_that!(&Fitness::abs_diff(&1u8, &0u8), is(equal_to(1u8)));
    expect_that!(&Fitness::abs_diff(&1u16, &0u16), is(equal_to(1u16)));
    expect_that!(&Fitness::abs_diff(&1u32, &0u32), is(equal_to(1u32)));
    expect_that!(&Fitness::abs_diff(&1u64, &0u64), is(equal_to(1u64)));
    expect_that!(&Fitness::abs_diff(&1usize, &0usize), is(equal_to(1usize)));
}

#[test]
fn abs_diff_of_unsigned_0_and_1() {
    expect_that!(&Fitness::abs_diff(&0u8, &1u8), is(equal_to(1u8)));
    expect_that!(&Fitness::abs_diff(&0u16, &1u16), is(equal_to(1u16)));
    expect_that!(&Fitness::abs_diff(&0u32, &1u32), is(equal_to(1u32)));
    expect_that!(&Fitness::abs_diff(&0u64, &1u64), is(equal_to(1u64)));
    expect_that!(&Fitness::abs_diff(&0usize, &1usize), is(equal_to(1usize)));
}

#[test]
fn abs_diff_of_unsigned_0_and_0() {
    expect_that!(&Fitness::abs_diff(&0u8, &0u8), is(equal_to(0u8)));
    expect_that!(&Fitness::abs_diff(&0u16, &0u16), is(equal_to(0u16)));
    expect_that!(&Fitness::abs_diff(&0u32, &0u32), is(equal_to(0u32)));
    expect_that!(&Fitness::abs_diff(&0u64, &0u64), is(equal_to(0u64)));
    expect_that!(&Fitness::abs_diff(&0usize, &0usize), is(equal_to(0usize)));
}

#[test]
fn abs_diff_of_unsigned_61_and_19() {
    expect_that!(&Fitness::abs_diff(&61u8, &19u8), is(equal_to(42u8)));
    expect_that!(&Fitness::abs_diff(&61u16, &19u16), is(equal_to(42u16)));
    expect_that!(&Fitness::abs_diff(&61u32, &19u32), is(equal_to(42u32)));
    expect_that!(&Fitness::abs_diff(&61u64, &19u64), is(equal_to(42u64)));
    expect_that!(
        &Fitness::abs_diff(&61usize, &19usize),
        is(equal_to(42usize))
    );
}

#[test]
fn abs_diff_of_unsigned_19_and_61() {
    expect_that!(&Fitness::abs_diff(&19u8, &61u8), is(equal_to(42u8)));
    expect_that!(&Fitness::abs_diff(&19u16, &61u16), is(equal_to(42u16)));
    expect_that!(&Fitness::abs_diff(&19u32, &61u32), is(equal_to(42u32)));
    expect_that!(&Fitness::abs_diff(&19u64, &61u64), is(equal_to(42u64)));
    expect_that!(
        &Fitness::abs_diff(&19usize, &61usize),
        is(equal_to(42usize))
    );
}

#[test]
fn abs_diff_of_unsigned_max_and_1() {
    expect_that!(
        &Fitness::abs_diff(&u8::MAX, &1u8),
        is(equal_to(u8::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&u16::MAX, &1u16),
        is(equal_to(u16::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&u32::MAX, &1u32),
        is(equal_to(u32::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&u64::MAX, &1u64),
        is(equal_to(u64::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&usize::MAX, &1usize),
        is(equal_to(usize::MAX - 1))
    );
}

#[test]
fn abs_diff_of_unsigned_1_and_max() {
    expect_that!(
        &Fitness::abs_diff(&1u8, &u8::MAX),
        is(equal_to(u8::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1u16, &u16::MAX),
        is(equal_to(u16::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1u32, &u32::MAX),
        is(equal_to(u32::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1u64, &u64::MAX),
        is(equal_to(u64::MAX - 1))
    );
    expect_that!(
        &Fitness::abs_diff(&1usize, &usize::MAX),
        is(equal_to(usize::MAX - 1))
    );
}
