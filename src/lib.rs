//! `genevo` is a library to search for an optimal solution by simulating the
//! evolution of data using a genetic algorithm.

#[cfg(test)] #[macro_use] extern crate quickcheck;
#[cfg(test)] #[macro_use] extern crate hamcrest;

extern crate chrono;
extern crate futures;
extern crate rand;

pub mod genetic;

pub mod simulation;

mod types;
