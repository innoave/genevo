//! Termination determines when to stop the process of the genetic algorithm.
//! Common termination conditions are:
//!
//! * A solution is found that satisfies minimum criteria
//! * A fixed number of generations is reached
//! * An allocated budget (computation time/money) is reached
//! * The highest ranking solution's fitness is reaching or has reached a
//!   plateau such that successive iterations no longer produce better results
//! * ...or a combination of such termination conditions.
//!
//! Termination conditions are defined by implementing the `Termination` trait.
//! Each kind of termination condition should be implemented as a separate
//! type. Terminations conditions are combined using combinators such as
//! logical and and logical or.
//!
//! A combinator is a special `Termination` condition that combines two other
//! `Termination` conditions with a logical operation. The most important
//! combinators are provided in the `combinator` package.
//!
//! For convenience the provided combinators can be instantiated by using
//! the public functions `and` and `or` which are reexported by this module.

pub mod combinator;
pub use self::combinator::{and, or};

pub mod limiter;
pub use self::limiter::*;

use genetic::{Fitness, Genotype, Phenotype};
use simulation::State;

/// A `Termination` defines a condition when the `Simulation` shall stop.
///
/// One implementation of the trait `Termination` should only handle one
/// single termination condition. In the simulation multiple termination
/// conditions can be combined through `combinator`s.
pub trait Termination<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// Evaluates whether the termination condition is met and returns true
    /// if the simulation shall be stopped or false if it shall continue.
    fn evaluate(&mut self, &state: State<'a, T, G, F>) -> bool;

    /// Resets the state of this `Termination` condition. This function is
    /// called on each `Termination` instance when the simulation is reset.
    ///
    /// This function needs to be implemented by an implementation of
    /// `Termination` if it has its own state, e.g. own counters.
    ///
    /// The default implementation does nothing.
    fn reset(&mut self) {}
}
