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

use genetic::{Breeding, Fitness, Genotype, ToScalar};
use operator::{GeneticOperator, SelectionOp, SingleObjective};
use rand::{Rng, thread_rng};
use simulation::{EvaluatedPopulation, SimError};
use std::marker::PhantomData;


/// The `RouletteWheelSelector` implements stochastic fitness proportionate
/// selection. Each candidate is picked randomly with a probability of being
/// picked that is proportional to its fitness value.
///
/// Characteristics: no bias, does not guarantee minimal spread.
#[derive(Clone)]
pub struct RouletteWheelSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// The breeding used to create parents.
    breeding: B,
    /// The number of parents to select.
    num_parents_to_select: usize,
    // phantom types
    _g: PhantomData<G>,
}

impl<G, B> RouletteWheelSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// Constructs a new instance of `RouletteWheelSelector`.
    pub fn new(breeding: B, num_parents_to_select: usize) -> RouletteWheelSelector<G, B> {
        RouletteWheelSelector {
            breeding: breeding,
            num_parents_to_select: num_parents_to_select,
            _g: PhantomData,
        }
    }

    /// Returns the `Breeding` used by this `RouletteWheelSelector`.
    pub fn breeding(&self) -> &B {
        &self.breeding
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
}

impl<G, B> SingleObjective for RouletteWheelSelector<G, B> where G: Genotype, B: Breeding<G> {}

impl<G, B> GeneticOperator for RouletteWheelSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    fn name() -> String {
        "Roulette-Wheel-Selection".to_string()
    }
}

impl<G, F, B> SelectionOp<G, F, B> for RouletteWheelSelector<G, B>
    where G: Genotype, F: Fitness + ToScalar, B: Breeding<G>
{
    fn selection(&self, evaluated: &EvaluatedPopulation<G, F>) -> Result<Vec<<B>::Parents>, SimError> {
        let mut parents = Vec::with_capacity(self.num_parents_to_select);
        let parents_size = self.breeding.num_individuals_per_parents();
        let (weights, weight_sum) = calc_cumulative_weight_distribution(evaluated.fitness_values());
        let mut rng = thread_rng();
        for _ in 0..self.num_parents_to_select {
            let mut tuple = Vec::with_capacity(parents_size);
            for _ in 0..parents_size {
                let random = rng.next_f64() * weight_sum;
                let selected = weighted_select(random, &weights);
                tuple.push(evaluated.individuals()[selected].clone());
            }
            parents.push(self.breeding.mate_parents(tuple));
        }
        Ok(parents)
    }
}

fn weighted_select(pointer: f64, weights: &[f64]) -> usize {
    let mut delta = pointer;
    for i in 0..weights.len() {
        delta -= weights[i];
        if delta <= 0. {
            return i;
        }
    }
    // when rounding errors occur, we return the last item's index
    return weights.len() - 1;
}

fn calc_cumulative_weight_distribution<F>(fitness_values: &[F]) -> (Vec<f64>, f64)
    where F: Fitness + ToScalar
{
    // cumulative weight distribution
    let mut cumulative_weights = Vec::with_capacity(fitness_values.len());
    let mut weight_sum: f64 = 0.;
    for i in 0..fitness_values.len() {
        let fitness = fitness_values[i].to_scalar();
        weight_sum = weight_sum + fitness;
        cumulative_weights.push(weight_sum);
    }
    (cumulative_weights, weight_sum)
}

/// The `UniversalSamplingSelector` implements stochastic fitness proportionate
/// selection. The first candidate is picked randomly. All other candidates are
/// picked by equidistant jumps.
///
/// Characteristics: no bias, minimal spread.
#[derive(Clone)]
pub struct UniversalSamplingSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    breeding: B,
    /// The number of parents to select.
    num_parents_to_select: usize,
    // phantom types
    _g: PhantomData<G>,
}

impl<G, B> UniversalSamplingSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// Constructs a new instance of `UniversalSamplingSelector`.
    pub fn new(breeding: B, num_parents_to_select: usize) -> UniversalSamplingSelector<G, B> {
        UniversalSamplingSelector {
            breeding: breeding,
            num_parents_to_select: num_parents_to_select,
            _g: PhantomData,
        }
    }

    /// Returns the `Breeding` used by this `UniversalSamplingSelector`.
    pub fn breeding(&self) -> &B {
        &self.breeding
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
}

impl<G, B> SingleObjective for UniversalSamplingSelector<G, B> where G: Genotype, B: Breeding<G> {}

impl<G, B> GeneticOperator for UniversalSamplingSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    fn name() -> String {
        "Stochastic-Universal-Sampling-Selection".to_string()
    }
}

impl<G, F, B> SelectionOp<G, F, B> for UniversalSamplingSelector<G, B>
    where G: Genotype, F: Fitness + ToScalar, B: Breeding<G>
{
    fn selection(&self, evaluated: &EvaluatedPopulation<G, F>) -> Result<Vec<<B>::Parents>, SimError> {
        let mut parents = Vec::with_capacity(self.num_parents_to_select);
        let parents_size = self.breeding.num_individuals_per_parents();
        let (weights, weight_sum) = calc_cumulative_weight_distribution(evaluated.fitness_values());
        let distance = weight_sum / (self.num_parents_to_select * parents_size) as f64;
        let mut pointer = thread_rng().next_f64() * weight_sum;
        for _ in 0..self.num_parents_to_select {
            let mut tuple = Vec::with_capacity(parents_size);
            for _ in 0..parents_size {
                let selected = weighted_select(pointer, &weights);
                tuple.push(evaluated.individuals()[selected].clone());
                pointer += distance;
            }
            parents.push(self.breeding.mate_parents(tuple));
        }
        Ok(parents)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
    use rand::{SeedableRng, StdRng};

    #[test]
    fn roulette_select_weighted_distribution() {
        let mut rng = StdRng::from_seed(&[42usize]);

        let weights = vec![200., 150., 600., 50.];
        let weight_sum = 1_000.;

        let mut counter = vec![0, 0, 0, 0];
        for _ in 0..weight_sum as usize {
            let random = rng.next_f64() * weight_sum;
            let index = weighted_select(random, &weights);
            counter[index] += 1;
        }

        assert_that!(counter[0], is(equal_to(204)));
        assert_that!(counter[1], is(equal_to(152)));
        assert_that!(counter[2], is(equal_to(600)));
        assert_that!(counter[3], is(equal_to(44)));
    }
}
