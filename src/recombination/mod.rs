//! The `recombination` module provides default implementations of the
//! `operator::CrossoverOp`. The provided crossover operators are organized
//! in the categories:
//! * `discrete` - crossover schemes working on discrete values of a bitset or
//!                or list of values.
//! * `order` - crossover schemes for permutation encoded values.

pub mod discrete;

pub mod order;
