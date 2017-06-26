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
//! genevo = "^0.1.0"
//! ```
//!
//! and adding `extern crate genevo;` to your crate root.

#[cfg(test)] #[macro_use] extern crate hamcrest;
#[cfg(test)] #[macro_use] extern crate quickcheck;

extern crate fixedbitset;
extern crate chrono;
extern crate jobsteal;
extern crate rand;


pub mod genetic;

pub mod encoding;

pub mod operator;

pub mod simulation;

pub mod selection;

pub mod recombination;

pub mod mutation;

pub mod reinsertion;

pub mod termination;

pub mod math;

pub mod random;

pub mod types;
