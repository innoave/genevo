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
use std::fmt;

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

implement_fitness_for_signed_integer!(i8, i16, i32, i64);


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

//TODO implement unit tests for implementations of the Fitness trait.

pub trait Display: fmt::Display {}

impl Display for Duration {

}
