//! The `limiter` package provides `Termination` functions that stop the
//! simulation when a certain limit is reached.
//!
//! Provided limiters are:
//! * `FitnessLimit` - stops the simulation after a solution with a certain
//!   fitness has been found.
//! * `IterationLimit` - stops the simulation after a maximum number of
//!   iterations has been processed.
//! * `TimeLimit` - stops the simulation after a the specified time limit
//!   has been reached.

use crate::{
    algorithm::Algorithm,
    ga::GeneticAlgorithm,
    genetic::{Fitness, FitnessFunction, Genotype},
    operator::{CrossoverOp, MutationOp, ReinsertionOp, SelectionOp},
    simulation::State,
    termination::{StopFlag, Termination},
};
use chrono::{Duration, Local};
use std::marker::PhantomData;

/// The `FitnessLimit` condition stops the simulation after a solution with
/// a certain fitness has been found.
#[derive(Clone, Debug, PartialEq)]
pub struct FitnessLimit<G, F>
where
    G: Genotype,
    F: Fitness,
{
    _g: PhantomData<G>,
    /// The fitness value that shall be reached to stop the simulation.
    fitness_target: F,
}

impl<G, F> FitnessLimit<G, F>
where
    G: Genotype,
    F: Fitness,
{
    /// Create a new instance of `FitnessLimit` with the specified limit
    /// of generations.
    pub fn new(fitness_target: F) -> Self {
        FitnessLimit {
            _g: PhantomData,
            fitness_target,
        }
    }

    /// Returns the fitness value that shall be reached to stop the simulation.
    pub fn fitness_target(&self) -> &F {
        &self.fitness_target
    }
}

impl<G, F, E, S, C, M, R> Termination<GeneticAlgorithm<G, F, E, S, C, M, R>> for FitnessLimit<G, F>
where
    G: Genotype,
    F: Fitness + Send + Sync,
    E: FitnessFunction<G, F>,
    E: FitnessFunction<G, F> + Sync,
    S: SelectionOp<G, F>,
    C: CrossoverOp<G> + Sync,
    M: MutationOp<G> + Sync,
    R: ReinsertionOp<G, F>,
{
    fn evaluate(&mut self, state: &State<GeneticAlgorithm<G, F, E, S, C, M, R>>) -> StopFlag {
        let highest_fitness = &state.result.best_solution.solution.fitness;
        if highest_fitness >= &self.fitness_target {
            StopFlag::StopNow(format!(
                "Simulation stopped after a solution with a fitness of {:?} \
                 has been found.",
                highest_fitness
            ))
        } else {
            StopFlag::Continue
        }
    }
}

/// The `GenerationLimit` condition stops the simulation after a maximum
/// number of generations has been processed.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct GenerationLimit {
    /// Maximum number of generations to process.
    max_generations: u64,
}

impl GenerationLimit {
    /// Create a new instance of `GenerationLimit` with the specified limit
    /// of generations.
    pub fn new(max_generations: u64) -> Self {
        GenerationLimit { max_generations }
    }

    pub fn max_generations(&self) -> &u64 {
        &self.max_generations
    }
}

impl<A> Termination<A> for GenerationLimit
where
    A: Algorithm,
{
    fn evaluate(&mut self, state: &State<A>) -> StopFlag {
        if state.iteration >= self.max_generations {
            StopFlag::StopNow(format!(
                "Simulation stopped after the limit of {} generations have \
                 been processed.",
                &state.iteration
            ))
        } else {
            StopFlag::Continue
        }
    }
}

/// The `TimeLimit` condition stops the simulation after the specified time
/// limit has been reached, i.e. the simulation is already running for the
/// specified amount of time.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct TimeLimit {
    /// Maximum time the simulation should run
    max_time: Duration,
}

impl TimeLimit {
    /// Create a new instance of `TimeLimit` with the specified amount
    /// of time.
    pub fn new(max_time: Duration) -> Self {
        TimeLimit { max_time }
    }

    /// Returns the maximum time the simulation should run.
    pub fn max_time(&self) -> &Duration {
        &self.max_time
    }
}

impl<A> Termination<A> for TimeLimit
where
    A: Algorithm,
{
    fn evaluate(&mut self, state: &State<A>) -> StopFlag {
        let duration = Local::now().signed_duration_since(state.started_at);
        if duration >= self.max_time {
            StopFlag::StopNow(format!(
                "Simulation stopped after running for {} which exceeds the \
                 maximal runtime of {}.",
                &duration, &self.max_time
            ))
        } else {
            StopFlag::Continue
        }
    }
}
