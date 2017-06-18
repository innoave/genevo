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
use genetic::{Fitness, Genotype, Phenotype};
use simulation::State;
use termination::Termination;
use std::marker::PhantomData;

/// The `GenerationLimit` condition stops the simulation after a maximum
/// number of generations has been processed.
pub struct GenerationLimit<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// Maximum number of generations to process.
    max_generations: u64,
    // phantom types
    phantom_t: &'a T,
    phantom_g: G,
    phantom_f: F,
}

impl<'a, T, G, F> GenerationLimit<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// Create a new instance of `GenerationLimit` with the specified limit
    /// of generations.
    pub fn new(max_generations: u64) -> Self {
        GenerationLimit {
            max_generations: max_generations,
            phantom_t: PhantomData::<T>,
            phantom_g: PhantomData::<G>,
            phantom_f: PhantomData::<F>,
        }
    }
}

impl<'a, T, G, F> Termination<'a, T, G, F> for GenerationLimit<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    fn evaluate(&self, &state: State<'a, T, G, F>) -> bool {
        state.generation >= self.max_generations
    }
}

/// The `TimeLimit` condition stops the simulation after the specified time
/// limit has been reached, i.e. the simulation is already running for the
/// specified amount of time.
pub struct TimeLimit<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// Maximum time the simulation should run
    max_time: Duration,
    // phantom types
    phantom_t: &'a T,
    phantom_g: G,
    phantom_f: F,
}

impl<'a, T, G, F> TimeLimit<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// Create a new instance of `TimeLimit` with the specified amount
    /// of time.
    pub fn new(max_time: Duration) -> Self {
        TimeLimit {
            max_time: max_time,
            phantom_t: PhantomData::<T>,
            phantom_g: PhantomData::<G>,
            phantom_f: PhantomData::<F>,
        }
    }
}

impl<'a, T, G, F> Termination<'a, T, G, F> for TimeLimit<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    fn evaluate(&self, &state: State<'a, T, G, F>) -> bool {
        Local::now().signed_duration_since(state.started_at) >= self.max_time
    }
}
