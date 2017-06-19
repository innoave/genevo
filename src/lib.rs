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

#[cfg(test)] extern crate test;
#[cfg(test)] #[macro_use] extern crate hamcrest;
#[cfg(test)] #[macro_use] extern crate quickcheck;

extern crate chrono;
extern crate futures;
extern crate rand;

pub mod breeding;

pub mod genetic;

pub mod mutation;

pub mod operator;

pub mod selection;

pub mod simulation;

pub mod termination;

mod types;
