//! The `random` module provides `operator::ReinsertionOp` that combine the
//! individuals from the offspring and the old population without considering
//! the fitness or any other attribute of the individuals.

use genetic::{Fitness, Genotype, Offspring};
use operator::{GeneticOperator, MultiObjective, ReinsertionOp, SingleObjective};
use simulation::{EvaluatedPopulation, SimError};
use random::random_index;
use rand::Rng;


/// The `UniformReinserter` takes n individuals from the offspring and
/// o individuals from the old population and combines them to the new
/// population. The sum of n and o is always equal to the size of the
/// old population. The individuals to be inserted in the new population
/// are picked uniformly at random.
///
/// The reinserter can be configured by the `replace_ratio` field. The
/// replace ratio is the fraction of the population size that is replaced by
/// individuals from the offspring. The remaining spots are filled with
/// individuals from the old population.
///
/// A replace ratio of 1.0 means that the new population is fist filled with
/// individuals from the offspring. if the offspring does not contain enough
/// individuals then the new population is filled up with individuals from the
/// old population. If the offspring contains more individuals than the size of
/// the population then the individuals are chosen uniformly at random.
#[derive(Clone)]
pub struct UniformReinserter {
    /// The `replace_ratio` defines the fraction of the population size that
    /// is going to be replaced by individuals from the offspring.
    replace_ratio: f64,
}

impl UniformReinserter {
    /// Constructs a new instance of the `UniformReinserter` with the given
    /// parameters.
    pub fn new(replace_ratio: f64) -> Self {
        UniformReinserter {
            replace_ratio: replace_ratio,
        }
    }

    /// Returns the `replace_ratio` of this `UniformReinserter`.
    pub fn replace_ratio(&self) -> f64 {
        self.replace_ratio
    }

    /// Set the `replace_ratio` of this `UniformReinserter` to the given
    /// value. The value must be between 0 and 1.0 (inclusive).
    pub fn set_replace_ratio(&mut self, value: f64) {
        self.replace_ratio = value;
    }
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
    fn combine<R>(&self, offspring: &mut Offspring<G>, evaluated: &EvaluatedPopulation<G, F>, rng: &mut R)
               -> Result<Vec<G>, SimError>
        where R: Rng + Sized {
        let old_individuals = evaluated.individuals();
        let population_size = old_individuals.len();
        let mut new_population = Vec::with_capacity(population_size);

        // How many individuals should we take from the offspring?
        let num_offspring = (population_size as f64 * self.replace_ratio + 0.5).floor() as usize;

        // first pick individuals from offspring
        if num_offspring < offspring.len() {
            // pick individuals from the offspring uniformly at random
            while num_offspring > new_population.len() {
                let index = random_index(rng, offspring.len());
                new_population.push(offspring.remove(index));
            }
        } else {
            // insert all individuals from offspring
            let mut i = 0;
            while i < offspring.len() {
                new_population.push(offspring.remove(i));
                i += 1;
            }
        }
        // finally fill up new population with individuals from old population
        // (as many as needed).
        let num_old_population = population_size - new_population.len();
        for _ in 0..num_old_population {
            let index = random_index(rng, old_individuals.len());
            new_population.push(old_individuals[index].clone());
        }
        Ok(new_population)
    }
}
