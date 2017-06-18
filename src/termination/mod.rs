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

/// The `StopFlag` is the result of the `Termination` function. It tells
/// the simulation whether it shall stop or if it can continue.
///
/// If the StopFlag indicates that the simulation must stop, also the reason
/// (`StopReason`) must be specified.
pub enum StopFlag {
    /// Flag for 'Stop the simulation now'.
    StopNow(StopReason),
    /// Flag for 'Continue with the simulation'.
    Continue,
}

/// A `StopReason` should explain to the user of the simulation why the
/// simulation has been stopped. Examples:
/// * "Simulation stopped after the maximum of 100 generations have been
///   processed"
/// * "Simulation stopped after a solution with a fitness value of 81 has
///   been found which is above the target fitness of 80.
pub type StopReason = String;

/// A `Termination` defines a condition when the `Simulation` shall stop.
///
/// One implementation of the trait `Termination` should only handle one
/// single termination condition. In the simulation multiple termination
/// conditions can be combined through `combinator`s.
pub trait Termination<'a, T, G, F>: Clone
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// Evaluates the termination condition and returns a `StopFlag` depending
    /// on the result. The `StopFlag` indicates whether the simulation shall
    /// stop or continue.
    ///
    /// In case the simulation shall be stopped, i.e. a `StopFlag::StopNow` is
    /// returned also a the reason why the simulation shall be stopped is
    /// returned. This reason should explain to the user of the simulation,
    /// why the simulation has been stopped.
    fn evaluate(&mut self, state: &State<'a, T, G, F>) -> StopFlag;

    /// Resets the state of this `Termination` condition. This function is
    /// called on each `Termination` instance when the simulation is reset.
    ///
    /// This function needs to be implemented by an implementation of
    /// `Termination` if it has its own state, e.g. own counters.
    ///
    /// The default implementation does nothing.
    fn reset(&mut self) {}
}
