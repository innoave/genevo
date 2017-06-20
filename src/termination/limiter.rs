//! The `limiter` package provides `Termination` functions that stop the
//! simulation when a certain limit is reached.
//!
//! Provided limiters are:
//!
//! * `GenerationLimit` - stops the simulation after a maximum number of
//!   generations has been processed.
//! * `TimeLimit` - stops the simulation after a the specified time limit
//!   has been reached.

use chrono::{Duration, Local};
use genetic::{Fitness, Genotype};
use simulation::State;
use termination::{StopFlag, Termination};
use std::marker::PhantomData;

/// The `GenerationLimit` condition stops the simulation after a maximum
/// number of generations has been processed.
#[derive(Clone)]
pub struct GenerationLimit<G, F>
    where G: Genotype, F: Fitness
{
    /// Maximum number of generations to process.
    max_generations: u64,
    // phantom types
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<G, F> GenerationLimit<G, F>
    where G: Genotype, F: Fitness
{
    /// Create a new instance of `GenerationLimit` with the specified limit
    /// of generations.
    pub fn new(max_generations: u64) -> Self {
        GenerationLimit {
            max_generations: max_generations,
            _g: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<G, F> Termination<G, F> for GenerationLimit<G, F>
    where G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: &State<G, F>) -> StopFlag {
        if state.generation >= self.max_generations {
            StopFlag::StopNow(format!("Simulation stopped after the limit of {} generation have \
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
pub struct TimeLimit<G, F>
    where G: Genotype, F: Fitness
{
    /// Maximum time the simulation should run
    max_time: Duration,
    // phantom types
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<G, F> TimeLimit<G, F>
    where G: Genotype, F: Fitness
{
    /// Create a new instance of `TimeLimit` with the specified amount
    /// of time.
    pub fn new(max_time: Duration) -> Self {
        TimeLimit {
            max_time: max_time,
            _g: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<G, F> Termination<G, F> for TimeLimit<G, F>
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
