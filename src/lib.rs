//! # genevo
//!
//! `genevo` is a library for implementing and executing simulations of
//! optimization and search problems using a genetic algorithm (GA).
//!
//! It provides a default implementation of the genetic algorithm to be used
//! to find solutions for a wide variety of search and optimization problems.
//!
//! The implementation is split into building blocks which are all represented by
//! traits. This crate provides most common implementation for all building blocks.
//! So it can be used for many problems out of the box.
//!
//! Anyway if one wants to use different implementations for one or the other
//! building block it can be extended by implementing any of the traits in a more
//! sophisticated and customized way.
//!
//! The building blocks (defined as traits) are:
//!
//! * Simulation
//! * Algorithm
//! * Termination
//! * Operator
//! * Population
//! * Phenotype and Genotype
//! * FitnessFunction
//!
//! The simulation can run an algorithm that is executed in a loop. An algorithm
//! implements the steps to be done for each iteration of the loop. The provided
//! implementation of the genetic algorithm implements the `Algorithm` trait and
//! can therefore be executed by the `Simulator` which is the provided
//! implementation of the `Simulation` trait.
//!
//! The `Simulator` holds state about the simulation and tracks statistics about
//! the execution of the algorithm, such as number of iterations and processing
//! time.
//!
//! The simulation runs until the termination criteria are met. The termination
//! criteria can be a single one such as max number of iterations or a logical
//! combination of multiple termination criteria, e.g. max number of iterations
//! OR a minimum fitness value has been reached. Of coarse `Termination` is a
//! trait as well and one can implement any termination criteria he/she can think
//! of.
//!
//! The algorithm can make use of operators that perform different stages of the
//! algorithm. E.g. the basic genetic algorithm defines the stages: selection,
//! crossover, mutation and accepting. These stages are performed by the appropriate
//! operators: `SelectionOp`, `CrossoverOp`, `MutationOp`, `RecombinationOp` and
//! `ReinsertionOp`.
//!
//! This crate provides multiple implementations for each one of those operators.
//! So one can experiment with combining the different implementations to compose
//! the best algorithm for a specific search or optimization problem. Now you may
//! have guessed that the defined operators are traits as well and you are free
//! to implement any of these operators in a way that suits best for your problem
//! and plug them into the provided implementation of the genetic algorithm.
//!
//! The genetic algorithm needs a population that it evolves with each iteration.
//! A population contains a number of individuals. Each individual represents a
//! possible candidate solution for an optimization problem for which the best
//! solution is searched for. This crate provides a `PopulationBuilder` to build
//! population of genomes. To run the population builder it needs an implementation
//! of the `GenomeBuilder` trait. A `GenomeBuilder` defines how to create one
//! individual (or genome) within the population.
//!
//! Last but maybe most important are the traits `Phenotype`, `Genotype` and
//! `FitnessFunction`. These are the traits which define the domain of the
//! optimization problem. They must be implemented individually for each application
//! of the genetic algorithm.
//!
//! Enough words about the building blocks. Show me some concrete examples. Have
//! a look at the examples in the examples folder to find out how to use this crate:
//!
//! * [monkeys](./examples/monkeys/main.rs): explores the idea of Shakespeare's monkeys, also known
//!   as the [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem)
//! * [queens](./examples/queens/main.rs): searches for solutions of the
//!   [N Queens Problem](https://en.wikipedia.org/wiki/Eight_queens_puzzle)

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

#[cfg(test)]
#[macro_use]
extern crate galvanic_assert;

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
