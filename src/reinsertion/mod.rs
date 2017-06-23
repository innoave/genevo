//! The `reinsertion` module provides implementations of the
//! `operator::ReinsertionOp` for basic strategies of reinsertion.
//!
//! The provided implementations of the `operator:ReinsertionOp` are:
//! * `UniformReinserter`
//! * `ElitistReinserter`

use genetic::{Fitness, Genotype};
use operator::{GeneticOperator, MultiObjective, ReinsertionOp, SingleObjective};
use simulation::{EvaluatedPopulation, SimError};


#[derive(Clone)]
pub struct UniformReinserter {

}

impl GeneticOperator for UniformReinserter {
    fn name() -> String {
        "Uniform-Reinserter".to_string()
    }
}

/// Can be used for single-objective optimization
impl SingleObjective for UniformReinserter {}
/// Can be used for multi-objective optimization
impl MultiObjective for UniformReinserter {}

impl<G, F> ReinsertionOp<G, F> for UniformReinserter
    where G: Genotype, F: Fitness
{
    fn combine(&self, offspring: &mut Vec<G>, evaluated: &EvaluatedPopulation<G, F>)
        -> Result<Vec<G>, SimError> {
        let old_individuals = evaluated.individuals();
        let population_size = old_individuals.len();
        let mut new_population = Vec::with_capacity(population_size);




        Ok(new_population)
    }
}
