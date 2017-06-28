//! The `limiter` package provides `Termination` functions that stop the
//! simulation when a certain limit is reached.
//!
//! Provided limiters are:
//! * `FitnessLimit` - stops the simulation after a solution with a certain
//!   fitness has been found.
//! * `GenerationLimit` - stops the simulation after a maximum number of
//!   generations has been processed.
//! * `TimeLimit` - stops the simulation after a the specified time limit
//!   has been reached.

use chrono::{Duration, Local};
use genetic::{Fitness, Genotype};
use simulation::State;
use termination::{StopFlag, Termination};


/// The `FitnessLimit` condition stops the simulation after a solution with
/// a certain fitness has been found.
#[derive(Clone)]
pub struct FitnessLimit<F>
    where F: Fitness
{
    /// The fitness value that shall be reached to stop simulation.
    fitness_target: F,
}

impl<F> FitnessLimit<F>
    where F: Fitness
{
    /// Create a new instance of `FitnessLimit` with the specified limit
    /// of generations.
    pub fn new(fitness_target: F) -> Self {
        FitnessLimit {
            fitness_target: fitness_target,
        }
    }
}

impl<G, F> Termination<G, F> for FitnessLimit<F>
    where G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: &State<G, F>) -> StopFlag {
        if state.highest_fitness >= self.fitness_target {
            StopFlag::StopNow(format!("Simulation stopped after a solution with a fitness of {:?} \
                has been found.", &state.highest_fitness))
        } else {
            StopFlag::Continue
        }
    }
}

/// The `GenerationLimit` condition stops the simulation after a maximum
/// number of generations has been processed.
#[derive(Clone)]
pub struct GenerationLimit {
    /// Maximum number of generations to process.
    max_generations: u64,
}

impl GenerationLimit {
    /// Create a new instance of `GenerationLimit` with the specified limit
    /// of generations.
    pub fn new(max_generations: u64) -> Self {
        GenerationLimit {
            max_generations: max_generations,
        }
    }
}

impl<G, F> Termination<G, F> for GenerationLimit
    where G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: &State<G, F>) -> StopFlag {
        if state.generation >= self.max_generations {
            StopFlag::StopNow(format!("Simulation stopped after the limit of {} generations have \
                been processed.", &state.generation))
        } else {
            StopFlag::Continue
        }
    }
}

/// The `TimeLimit` condition stops the simulation after the specified time
/// limit has been reached, i.e. the simulation is already running for the
/// specified amount of time.
#[derive(Clone)]
pub struct TimeLimit {
    /// Maximum time the simulation should run
    max_time: Duration,
}

impl TimeLimit {
    /// Create a new instance of `TimeLimit` with the specified amount
    /// of time.
    pub fn new(max_time: Duration) -> Self {
        TimeLimit {
            max_time: max_time,
        }
    }
}

impl<G, F> Termination<G, F> for TimeLimit
    where G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: &State<G, F>) -> StopFlag {
        let duration = Local::now().signed_duration_since(state.started_at);
        if duration >= self.max_time {
            StopFlag::StopNow(format!("Simulation stopped after running for {} which exceeds the \
                maximal runtime of {}.", &duration, &self.max_time))
        } else {
            StopFlag::Continue
        }
    }
}
