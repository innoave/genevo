//! The `truncation` module provides selection methods that are merely based
//! on the fitness values of the individuals but don't have any stochastic
//! component.
//!
//! The provided `SelectionOp` implementations are:
//! * `MaximizeSelector`

use genetic::{Breeding, Fitness, Genotype};
use operator::{GeneticOperator, SelectionOp, SingleObjective, MultiObjective};
use simulation::{EvaluatedPopulation, SimError};
use std::marker::PhantomData;


/// The `MaximizeSelector` selects the best performing `genetic::Genotype`s
/// from the population.
///
/// This `MaximizeSelector` can be used for single-objective fitness values
/// as well as multi-objective fitness values.
#[derive(Clone)]
pub struct MaximizeSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// The breeding used to create parents.
    breeding: B,
    /// The truncation threshold is the ratio between the number of parents
    /// to be selected and the size of the population:
    /// threshold = number of parents / size of population
    truncation_threshold: f64,
    // phantom types
    _g: PhantomData<G>,
}

impl<G, B> MaximizeSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// Constructs a new instance of the `MaximizeSelector`.
    pub fn new(breeding: B, truncation_threshold: f64) -> MaximizeSelector<G, B> {
        MaximizeSelector {
            breeding: breeding,
            truncation_threshold: truncation_threshold,
            _g: PhantomData,
        }
    }

    /// Returns the `Breeding` used by this `MaximizeSelector`.
    pub fn breeding(&self) -> &B {
        &self.breeding
    }

    /// Returns the truncation threshold used by this `MaximizeSelector`.
    ///
    /// The truncation threshold is the ratio between the number of parents
    /// to be selected and the size of the population:
    /// threshold = number of parents / size of population
    pub fn truncation_threshold(&self) -> f64 {
        self.truncation_threshold
    }

    /// Sets the truncation threshold to the given value.
    ///
    /// The truncation threshold is the ratio between the number of parents
    /// to be selected and the size of the population:
    /// threshold = number of parents / size of population
    pub fn set_truncation_threshold(&mut self, value: f64) {
        self.truncation_threshold = value;
    }
}

/// Can be used for single-objective optimization
impl<G, B> SingleObjective for MaximizeSelector<G, B> where G: Genotype, B: Breeding<G> {}
/// Can be used for multi-objective optimization
impl<G, B> MultiObjective for MaximizeSelector<G, B> where G: Genotype, B: Breeding<G> {}

impl<G, B> GeneticOperator for MaximizeSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    fn name() -> String {
        "Maximizing-Truncation-Selection".to_string()
    }
}

impl<G, F, B> SelectionOp<G, F, B> for MaximizeSelector<G, B>
    where G: Genotype, F: Fitness, B: Breeding<G>
{
    fn selection(&self, evaluated: &EvaluatedPopulation<G, F>) -> Result<Vec<B::Parents>, SimError> {
        let individuals = evaluated.individuals();
        let fitness_values = evaluated.fitness_values();

        // mating pool holds indices to the individuals and fitness_values slices
        let mut mating_pool: Vec<usize> = (0..fitness_values.len()).collect();
        // sort mating pool from best performing to worst performing index
        mating_pool.sort_by(|x, y| fitness_values[*y].cmp(&fitness_values[*x]));
        let mating_pool = mating_pool;

        let num_parents_to_select = (individuals.len() as f64 * self.truncation_threshold).floor() as usize;
        let parents_size = self.breeding.num_individuals_per_parents();
        let pool_size = mating_pool.len();
        let mut selected: Vec<B::Parents> = Vec::with_capacity(num_parents_to_select);

        let mut index_m = 0;
        for _ in 0..num_parents_to_select {
            let mut tuple = Vec::with_capacity(parents_size);
            for _ in 0..parents_size {
                // index into mating pool
                index_m = index_m % pool_size;
                // index into individuals slice
                let index_i = mating_pool[index_m];
                tuple.push(individuals[index_i].clone());
                index_m += 1;
            }
            selected.push(self.breeding.mate_parents(tuple));
        }
        Ok(selected)
    }
}
