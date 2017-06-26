//! The `proportional` module provides `operator::SelectionOp`s that implement
//! stochastic fitness proportionate selection strategies. Individuals are
//! randomly selected. Individuals with a higher `Fitness` value are having a
//! higher probability to be selected.
//!
//! How is this achieved?
//! In fitness proportionate selection each individual gets assigned a weight
//! that is equal to its fitness value plus the sum of fitness values of all
//! individuals in the list before him (cumulative weight distribution). Then
//! a uniform random number between 0 and the sum of all weights is used to
//! select a candidate.
//!
//! The provided **fitness proportionate selection** operators are:
//! * `RouletteWheelSelector` - no bias - does not guarantee minimal spread.
//! * `UniversalSamplingSelector` - no bias - minimal spread.

use genetic::{Fitness, Genotype, Parents, ToScalar};
use math::WeightedDistribution;
use operator::{GeneticOperator, SelectionOp, SingleObjective};
use random::random_probability;
use rand::thread_rng;
use simulation::{EvaluatedPopulation, SimError};


/// The `RouletteWheelSelector` implements stochastic fitness proportionate
/// selection. Each candidate is picked randomly with a probability of being
/// picked that is proportional to its fitness value.
///
/// Characteristics: no bias, does not guarantee minimal spread.
#[derive(Clone)]
pub struct RouletteWheelSelector {
    /// The number of parents to select.
    num_parents_to_select: usize,
    /// The number of individuals per parents.
    num_individuals_per_parents: usize,
}

impl RouletteWheelSelector {
    /// Constructs a new instance of `RouletteWheelSelector`.
    pub fn new(num_parents_to_select: usize, num_individuals_per_parents: usize) -> Self {
        RouletteWheelSelector {
            num_parents_to_select: num_parents_to_select,
            num_individuals_per_parents: num_individuals_per_parents,
        }
    }

    /// Returns the number of parents that are selected on every call of the
    /// `selection` function.
    pub fn num_parents_to_select(&self) -> usize {
        self.num_parents_to_select
    }

    /// Sets the number of parents that are selected on every call of the
    /// `selection` function to a new value.
    pub fn set_num_parents_to_select(&mut self, value: usize) {
        self.num_parents_to_select = value;
    }

    /// Returns the number of individuals per parents use by this selector.
    pub fn num_individuals_per_parents(&self) -> usize {
        self.num_individuals_per_parents
    }

    /// Sets the number of individuals per parents to the given value.
    pub fn set_num_individuals_per_parents(&mut self, value: usize) {
        self.num_individuals_per_parents = value;
    }
}

impl SingleObjective for RouletteWheelSelector {}

impl GeneticOperator for RouletteWheelSelector {
    fn name() -> String {
        "Roulette-Wheel-Selection".to_string()
    }
}

impl<G, F> SelectionOp<G, F> for RouletteWheelSelector
    where G: Genotype, F: Fitness + ToScalar
{
    fn select_from(&self, evaluated: &EvaluatedPopulation<G, F>) -> Result<Vec<Parents<G>>, SimError> {
        let mut parents = Vec::with_capacity(self.num_parents_to_select);
        let weighted_distribution = WeightedDistribution::from_scalar_values(evaluated.fitness_values());
        let mut rng = thread_rng();
        for _ in 0..self.num_parents_to_select {
            let mut tuple = Vec::with_capacity(self.num_individuals_per_parents);
            for _ in 0..self.num_individuals_per_parents {
                let random = random_probability(&mut rng) * weighted_distribution.sum();
                let selected = weighted_distribution.select(random);
                tuple.push(evaluated.individuals()[selected].clone());
            }
            parents.push(tuple);
        }
        Ok(parents)
    }
}

/// The `UniversalSamplingSelector` implements stochastic fitness proportionate
/// selection. The first candidate is picked randomly. All other candidates are
/// picked by equidistant jumps.
///
/// Characteristics: no bias, minimal spread.
#[derive(Clone)]
pub struct UniversalSamplingSelector {
    /// The number of parents to select.
    num_parents_to_select: usize,
    /// The number of individuals per parents.
    num_individuals_per_parents: usize,
}

impl UniversalSamplingSelector {
    /// Constructs a new instance of `UniversalSamplingSelector`.
    pub fn new(num_parents_to_select: usize, num_individuals_per_parents: usize) -> Self {
        UniversalSamplingSelector {
            num_parents_to_select: num_parents_to_select,
            num_individuals_per_parents: num_individuals_per_parents,
        }
    }

    /// Returns the number of parents that are selected on every call of the
    /// `selection` function.
    pub fn num_parents_to_select(&self) -> usize {
        self.num_parents_to_select
    }

    /// Sets the number of parents that are selected on every call of the
    /// `selection` function to a new value.
    pub fn set_num_parents_to_select(&mut self, value: usize) {
        self.num_parents_to_select = value;
    }

    /// Returns the number of individuals per parents use by this selector.
    pub fn num_individuals_per_parents(&self) -> usize {
        self.num_individuals_per_parents
    }

    /// Sets the number of individuals per parents to the given value.
    pub fn set_num_individuals_per_parents(&mut self, value: usize) {
        self.num_individuals_per_parents = value;
    }
}

impl SingleObjective for UniversalSamplingSelector {}

impl GeneticOperator for UniversalSamplingSelector {
    fn name() -> String {
        "Stochastic-Universal-Sampling-Selection".to_string()
    }
}

impl<G, F> SelectionOp<G, F> for UniversalSamplingSelector
    where G: Genotype, F: Fitness + ToScalar
{
    fn select_from(&self, evaluated: &EvaluatedPopulation<G, F>) -> Result<Vec<Parents<G>>, SimError> {
        let mut parents = Vec::with_capacity(self.num_parents_to_select);
        let weighted_distribution = WeightedDistribution::from_scalar_values(evaluated.fitness_values());
        let distance = weighted_distribution.sum() / (self.num_parents_to_select * self.num_individuals_per_parents) as f64;
        let mut rng = thread_rng();
        let mut pointer = random_probability(&mut rng) * weighted_distribution.sum();
        for _ in 0..self.num_parents_to_select {
            let mut tuple = Vec::with_capacity(self.num_individuals_per_parents);
            for _ in 0..self.num_individuals_per_parents {
                let selected = weighted_distribution.select(pointer);
                tuple.push(evaluated.individuals()[selected].clone());
                pointer += distance;
            }
            parents.push(tuple);
        }
        Ok(parents)
    }
}
