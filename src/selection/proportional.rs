//! The `proportional` module provides `operator::SelectionOp`s that implement
//! selection strategies where individuals with a higher `Fitness` value are
//! selected with a proportional higher probability.
//!
//! The provided selection operators are:
//! * `RouletteWheelSelector`

use genetic::{Breeding, Fitness, Genotype, ToScalar};
use operator::{GeneticOperator, SelectionOp, SingleObjective};
use rand::{Rng, thread_rng};
use simulation::EvaluatedPopulation;
use std::marker::PhantomData;

const OPERATOR_NAME: &str = "Roulette-Wheel-Selection";

/// The `RouletteWheelSelector` implements fitness proportionate selection.
#[derive(Clone)]
pub struct RouletteWheelSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// The type object used to know how to 'breed'.
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
        OPERATOR_NAME.to_string()
    }
}

impl<G, F, B> SelectionOp<G, F, B> for RouletteWheelSelector<G, B>
    where G: Genotype, F: Fitness + ToScalar, B: Breeding<G>
{
    fn selection(&self, evaluated: &EvaluatedPopulation<G, F>) -> Vec<<B>::Parents> {
        let mut parents = Vec::with_capacity(self.num_parents_to_select);
        let parents_size = self.breeding.num_individuals_per_parents();
        let (weights, weight_sum) = calculate_accumulated_weights(evaluated.fitness_values());
        let mut rng = thread_rng();
        for _ in 0..self.num_parents_to_select {
            let mut tuple = Vec::with_capacity(parents_size);
            for _ in 0..parents_size {
                let selected = roulette_select(&mut rng, &weights, weight_sum);
                tuple.push(evaluated.individuals()[selected].clone());
            }
            parents.push(self.breeding.mate_parents(tuple));
        }
        parents
    }
}

fn roulette_select<R>(rng: &mut R, weights: &[f64], weight_sum: f64) -> usize
    where R: Rng + Sized
{
    let mut random = rng.next_f64() * weight_sum;
    for i in 0..weights.len() {
        random -= weights[i];
        if random <= 0. {
            return i;
        }
    }
    // when rounding errors occur, we return the last item's index
    return weights.len() - 1;
}

fn calculate_accumulated_weights<F>(fitness_values: &[F]) -> (Vec<f64>, f64)
    where F: Fitness + ToScalar
{
    let mut accumulated_weights = Vec::with_capacity(fitness_values.len());
    let mut weight_sum: f64 = 0.;
    for i in 0..fitness_values.len() {
        let fitness = fitness_values[i].to_scalar();
        weight_sum = weight_sum + fitness;
        accumulated_weights.push(weight_sum);
    }
    (accumulated_weights, weight_sum)
}


#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn roulette_select_weighted_distribution() {
        let mut rng = thread_rng();

        let weights = vec![200., 150., 600., 50.];
        let weight_sum = 1_000.;

        let mut counter = vec![0, 0, 0, 0];
        for _ in 0..weight_sum as usize {
            let index = roulette_select(&mut rng, &weights, weight_sum);
            counter[index] += 1;
        }

        assert_that!(counter[0], is(greater_than(175)));
        assert_that!(counter[0], is(less_than(225)));
        assert_that!(counter[1], is(greater_than(125)));
        assert_that!(counter[1], is(less_than(175)));
        assert_that!(counter[2], is(greater_than(550)));
        assert_that!(counter[2], is(less_than(650)));
        assert_that!(counter[3], is(greater_than(40)));
        assert_that!(counter[3], is(less_than(60)));
    }
}
