//! # genevo
//!
//! `genevo` is a library for implementing and executing simulations of
//! optimization and search problems using a genetic algorithm (GA).
//!
//! ## Installation
//!
//! You can use this library by adding the following lines to your `Cargo.toml`
//! file:
//!
//! ```ignore
//! [dependencies]
//! genevo = "0.1"
//! ```
//!
//! and adding `extern crate genevo;` to your crate root.

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
)]

#[cfg(test)] #[macro_use] extern crate hamcrest;
#[cfg(test)] #[macro_use] extern crate quickcheck;

extern crate fixedbitset;
extern crate chrono;
extern crate rand;
extern crate rayon;
extern crate xorshift;

pub mod prelude;

pub mod genetic;

pub mod algorithm;

pub mod ga;

pub mod population;

pub mod encoding;

pub mod operator;

pub mod simulation;

pub mod selection;

pub mod recombination;

pub mod mutation;

pub mod reinsertion;

pub mod termination;

pub mod random;

pub mod statistic;

pub mod types;
